// Runs at the start of every frame, before the world and the HUD are drawn.
// It keeps the mod's skin in the atlas and tells the game's own skin lookup
// (`default_skin`, where every unresolved skin falls back) to answer with it
// for the local player. That lookup runs while the player list is built, in
// the HUD and before any screen, so the body, the first-person arm and the
// survival and creative inventory previews all get the same skin.

fn horizon_skin_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::horizon_skin as m;

    let settings = m::settings(|key| game.native_setting(m::ID, key));
    if !game.native_on(m::ID) || !settings.enabled {
        // Turned off: give the model back to the game, once.
        horizon_client::tab_list::horizon_skin_hub::clear_local();
        if m::set_wearing(false)
            && let Some(model) = game.player_model.as_mut()
        {
            model.skin = None;
        }
        return;
    }

    // A PNG is read once per file version; a bad one is said in chat once.
    let read = |path: &std::path::Path| -> Result<Vec<u8>, String> {
        let bytes = std::fs::read(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let (width, height, rgba) = horizon_client::hud::decode_bytes(&bytes)
            .map_err(|error| format!("{} is not a PNG Horizon reads: {error}", path.display()))?;
        horizon_client::skins::process_legacy_skin(width, height, &rgba).ok_or_else(|| {
            format!(
                "{} is {width}x{height}: a skin is 64x64 or 64x32",
                path.display()
            )
        })
    };

    // `file` names the PNG `/skin <file>` copied into the mod's folder; with
    // none, the folder's own skin.png.
    let path = game
        .native_setting(m::ID, "file")
        .map(|file| file.trim().to_owned())
        .filter(|file| !file.is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| game.native_data_dir(m::ID).join(m::FILE));
    let key = path.to_string_lossy().into_owned();
    let stamp = std::fs::metadata(&path)
        .and_then(|meta| meta.modified())
        .ok();

    let (rgba, fresh) = match m::cached_pixels(&key, stamp) {
        Some(rgba) => (rgba, false),
        None => match read(&path) {
            Ok(rgba) => {
                m::remember_pixels(key, stamp, rgba.clone());
                (rgba, true)
            }
            Err(error) => {
                if m::new_error(&error) {
                    game.native_chat(&format!("Custom Skin: {error}"));
                }
                return;
            }
        },
    };
    m::clear_error();

    // A slim skin lives under a name the game reads as a slim model, so the
    // fallback keeps the arms narrow. The atlas uploads the image through a
    // skin slot; a new file needs it again even when the sprite is known.
    let location = if settings.model.slim() {
        m::SLIM_LOCATION
    } else {
        m::LOCATION
    };
    if fresh || !game.gui.atlas.sprites.contains_key(location) {
        game.store_skin(location.to_string(), rgba);
    }
    // Keep the slot: an undrawn skin is the first the game gives away.
    game.skin_downloads.borrow_mut().drawn(location);

    // The local player's UUID: the player list resolves its skin every frame.
    if let Some(uuid) = game.remote.as_ref().map(|r| r.profile()) {
        horizon_client::tab_list::horizon_skin_hub::set_local(uuid, location);
    } else {
        horizon_client::tab_list::horizon_skin_hub::clear_local();
    }
    if let Some(model) = game.player_model.as_mut() {
        model.skin = Some(location.to_string());
        model.slim = settings.model.slim();
    }
    m::set_wearing(true);
}

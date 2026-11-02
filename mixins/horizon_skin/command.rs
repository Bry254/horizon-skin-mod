// `/skin` and its options: the PNG to wear, the arms, on/off, reload.
// `/skin <file.png>` copies the file into the mod's folder, so the skin
// survives the original moving or being deleted.

fn horizon_skin_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::horizon_skin as m;

    let rest = match command.strip_prefix("skin") {
        Some(rest) => rest,
        None => match command.strip_prefix("hskin") {
            Some(rest) => rest,
            None => return,
        },
    };
    if !(rest.is_empty() || rest.starts_with(' ')) || !game.native_on(m::ID) {
        return;
    }
    *ran = true;

    let args = rest.trim();
    let option = args.to_ascii_lowercase();
    let mut settings = m::settings(|key| game.native_setting(m::ID, key));
    // The PNG the mod reads now: the `file` setting, or the folder's skin.png.
    let file = game
        .native_setting(m::ID, "file")
        .map(|file| file.trim().to_owned())
        .filter(|file| !file.is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| game.native_data_dir(m::ID).join(m::FILE));

    match option.as_str() {
        "" => {
            game.native_chat(&format!(
                "Custom Skin {}: {}, arms {}. Skin: {}",
                m::VERSION,
                if settings.enabled { "on" } else { "off" },
                settings.model.label(),
                file.display()
            ));
            game.native_chat(
                "Use /skin <file.png>, /skin on, /skin off, /skin slim, /skin classic or /skin reload.",
            );
        }
        "on" => {
            settings.enabled = true;
            game.native_chat("Custom Skin: on.");
        }
        "off" => {
            settings.enabled = false;
            game.native_chat("Custom Skin: off, your account's skin shows again.");
        }
        "slim" => {
            settings.model = m::Model::Slim;
            game.native_chat("Custom Skin: slim arms.");
        }
        "classic" => {
            settings.model = m::Model::Classic;
            game.native_chat("Custom Skin: classic arms.");
        }
        "reload" => {
            m::forget_pixels();
            m::clear_error();
            game.native_chat(&format!(
                "Custom Skin: reading {} again.",
                file.display()
            ));
        }
        _ => {
            let source = std::path::PathBuf::from(args.trim_matches('"').trim_matches('\''));
            // Check it is a skin Horizon can lay out, copy it into the mod's
            // folder as skin.png, and point the `file` setting at it.
            let installed: Result<std::path::PathBuf, String> = (|| {
                let bytes = std::fs::read(&source)
                    .map_err(|error| format!("cannot read {}: {error}", source.display()))?;
                let (width, height, rgba) = horizon_client::hud::decode_bytes(&bytes).map_err(
                    |error| format!("{} is not a PNG Horizon reads: {error}", source.display()),
                )?;
                if horizon_client::skins::process_legacy_skin(width, height, &rgba).is_none() {
                    return Err(format!(
                        "{} is {width}x{height}: a skin is 64x64 or 64x32",
                        source.display()
                    ));
                }
                let dir = game.native_data_dir(m::ID);
                std::fs::create_dir_all(&dir)
                    .map_err(|error| format!("cannot make {}: {error}", dir.display()))?;
                let dest = dir.join(m::FILE);
                if source != dest {
                    std::fs::copy(&source, &dest)
                        .map_err(|error| format!("cannot copy to {}: {error}", dest.display()))?;
                }
                game.native_set_setting(m::ID, "file", &dest.to_string_lossy());
                m::forget_pixels();
                m::clear_error();
                Ok(dest)
            })();
            match installed {
                Ok(dest) => {
                    settings.enabled = true;
                    game.native_chat(&format!("Custom Skin: wearing {}.", dest.display()));
                }
                Err(error) => game.native_chat(&format!("Custom Skin: {error}")),
            }
        }
    }

    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}

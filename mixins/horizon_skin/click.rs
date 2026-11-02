// A click on the settings page: change the setting, keep it, and read the
// skin again when the reload button is pressed.
fn horizon_skin_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::horizon_skin as m;
    if mod_id != m::ID {
        return;
    }
    let mut settings = m::settings(|key| game.native_setting(m::ID, key));
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
    if button >= 2 {
        m::forget_pixels();
        m::clear_error();
    }
}

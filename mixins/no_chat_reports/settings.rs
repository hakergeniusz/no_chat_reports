// The mod's settings page: the six switch rows, in the state the files hold
// (read fresh through the game's own setting reader, so the page always
// shows what the gates act on).
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use horizon_client::lan::hub::no_chat_reports as ncr;
    if mod_id != ncr::ID {
        return;
    }
    let game = &*game;
    let sw = ncr::parse(|key| game.native_setting(ncr::ID, key));
    for i in 0..ncr::ROWS.len() {
        if let Some(label) = ncr::label(i, &sw) {
            buttons.push(label);
        }
    }
}

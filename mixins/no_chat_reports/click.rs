// A click on the mod's settings page: flip the row's switch and keep it in
// the setting file every gate re-reads, so the change lands within the tick
// that follows the click — on this game's client gates and, on a hosted
// server, on the relay gates that read the same folder.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client::lan::hub::no_chat_reports as ncr;
    if mod_id != ncr::ID {
        return;
    }
    let clicked = {
        let game = &*game;
        let sw = ncr::parse(|key| game.native_setting(ncr::ID, key));
        ncr::click(button, &sw)
    };
    if let Some((key, on)) = clicked {
        let (key, value) = ncr::stored(key, on);
        game.native_set_setting(ncr::ID, key, &value);
    }
}

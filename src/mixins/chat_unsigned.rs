// The chat line's gate: while Disable Signing is on, any session the client
// still holds — one made before the switch, or while it was off — is dropped
// before the send body reads it, so the line goes out with no signature. The
// body then does everything else as vanilla: the 256-unit trim, the
// last-seen update, the unsigned packet.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_unsigned(game: &mut crate::game::Game<'_>) {
    let sw = no_chat_reports::switches();
    if no_chat_reports::signing_blocked(&sw) {
        if let Some(r) = game.remote.as_mut() {
            r.secure_chat.session = None;
        }
    }
}

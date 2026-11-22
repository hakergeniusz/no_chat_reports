// The chat session's gate: while Disable Signing is on, the client never
// prepares a session — the `ChatSessionUpdate` with the account's public key
// is never sent — and any session still held from before the switch is
// dropped, so nothing on this client signs. With the switch off, the
// engine's own session setup runs untouched.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_session(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    let sw = no_chat_reports::switches();
    if no_chat_reports::signing_blocked(&sw) {
        if let Some(r) = game.remote.as_mut() {
            r.secure_chat.session = None;
        }
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}

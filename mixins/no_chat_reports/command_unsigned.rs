// The command's gate: while Disable Signing is on, any session the client
// still holds is dropped before the send body reads it, so a command's
// message arguments go out unsigned — `ChatCommandSigned` with an empty
// signature list, exactly as a client without a session sends them.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_command(game: &mut crate::game::Game<'_>) {
    let sw = horizon_client::lan::hub::no_chat_reports::switches();
    if horizon_client::lan::hub::no_chat_reports::signing_blocked(&sw)
        && let Some(r) = game.remote.as_mut()
    {
        r.secure_chat.session = None;
    }
}

// The relay's gate: the per-guest send of a player message in
// `broadcast_chat`. Two switches answer before the engine's own send:
//
// Strip Signatures — the message goes on unsigned: a copy without the
// signature, the chain index or the last-seen list, exactly the
// `ChatMessage::new` shape of an unsigned message, so no receiving client
// ever holds a verifiable, reportable line and the hub keeps no
// pending-signature bookkeeping for it. A server with
// `enforces_secure_chat` still refuses what the sender signed beyond its
// own policy — the switch cannot void the server's own setting, the same
// as upstream.
//
// Convert To System Messages — the message reaches the guest as a system
// line instead: `chat.type.text` with the sender's display name and the
// content, what the player chat type decorates on a client, so a client
// that would badge an unsigned line has nothing signed-shaped to badge
// (upstream convertToGameMessage).
//
// With both switches off, the engine's own send runs untouched.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_relay(
    hub: &mut Hub,
    k: usize,
    message: &ChatMessage,
    bound: &play::ChatTypeBound,
    original: impl FnOnce(&mut Hub, usize, &ChatMessage, &play::ChatTypeBound) -> (),
) {
    use crate::lan::hub::no_chat_reports as ncr;
    let sw = ncr::switches();
    if !ncr::relay_stripped(&sw) && !ncr::relay_converted(&sw) {
        return original(hub, k, message, bound);
    }
    let unsigned;
    let message: &ChatMessage = if ncr::relay_stripped(&sw) && message.signature.is_some() {
        unsigned = ChatMessage {
            signature: None,
            index: 0,
            last_seen: Vec::new(),
            ..message.clone()
        };
        &unsigned
    } else {
        message
    };
    if !ncr::relay_converted(&sw) {
        return original(hub, k, message, bound);
    }
    let name = hub.player_display_name(message.sender, &hub.guests[k].name);
    let g = &mut hub.guests[k];
    if g.info.chat_visibility != 0 {
        return;
    }
    let content = Tag::compound([
        ("translate", Tag::String(JavaString::from("chat.type.text"))),
        ("with", Tag::List(vec![name, literal(&message.content)])),
    ]);
    g.send(&C::SystemChat(play::SystemChat {
        content,
        overlay: false,
    }));
}

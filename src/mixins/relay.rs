// The relay's gate: the LAN host's broadcast of a player message. Two
// switches answer before the engine's own broadcast:
//
// Strip Signatures — the message goes on unsigned: a copy without the
// signature, the chain index or the last-seen list, exactly the shape of an
// unsigned message, so no receiving client ever holds a verifiable,
// reportable line and the hub keeps no pending-signature bookkeeping for it.
// A server with `enforces_secure_chat` still refuses what the sender signed
// beyond its own policy — the switch cannot void the server's own setting,
// the same as upstream. The host's own display comes back unsecure with
// their own line, as it does for a sessionless client's sends.
//
// Convert To System Messages — the message reaches every visible guest as a
// system line instead: `chat.type.text` with the sender's display name and
// the content, what the player chat type decorates on a client, so a client
// that would badge an unsigned line has nothing signed-shaped to badge
// (upstream convertToGameMessage).
//
// With both switches off, the engine's own broadcast runs untouched.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_relay(
    game: &mut crate::game::Game<'_>,
    message: &crate::lan::hub::chat::ChatMessage,
    bound: &horizon_protocol::packets::play::ChatTypeBound,
    line: &str,
    name: &str,
    shown: &str,
) -> core::ops::ControlFlow<Option<()>> {
    let sw = no_chat_reports::switches();
    let strip = no_chat_reports::relay_stripped(&sw);
    let convert = no_chat_reports::relay_converted(&sw);
    if !strip && !convert {
        return core::ops::ControlFlow::Continue(());
    }
    let hidden = game.chat_hidden_from(message.sender, name, shown);
    let Some(host) = game.lan.as_mut() else {
        return core::ops::ControlFlow::Break(None);
    };
    let mut chat = Vec::new();
    if convert {
        // The host's own display keeps the original line, trusted or not.
        chat.push(HostEvent::Chat {
            line: line.to_owned(),
            secure: message.trusted(crate::lan::hub::chat::now_millis()),
        });
        let mut sends = Vec::new();
        host.hub.guests.update_all(|g| {
            if g.placed && !hidden.contains(&g.uuid) && g.info.chat_visibility == 0 {
                sends.push((g.uuid, g.name.clone()));
            }
        });
        use horizon_core::nbt::{JavaString, Tag};
        for (uuid, guest_name) in sends {
            let display = host.hub.player_display_name(uuid, &guest_name);
            let content = Tag::compound([
                (
                    "translate",
                    Tag::String(JavaString::from("chat.type.text")),
                ),
                (
                    "with",
                    Tag::List(vec![
                        display,
                        Tag::String(JavaString::from(&*message.content)),
                    ]),
                ),
            ]);
            host.hub.system_message_to_player(uuid, content);
        }
        for event in chat {
            game.lan_log_event(event);
        }
        return core::ops::ControlFlow::Break(Some(()));
    }
    // Strip only: the broadcast runs on the unsigned copy, per guest and for
    // the host's own log.
    let unsigned;
    let message: &ChatMessage = if message.signature.is_some() {
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
    host.hub.broadcast_chat(message, bound, line.to_owned(), &hidden, &mut chat);
    for event in chat {
        game.lan_log_event(event);
    }
    core::ops::ControlFlow::Break(Some(()))
}

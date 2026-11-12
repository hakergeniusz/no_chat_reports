# No Chat Reports

A Horizon native mod: chat goes out unsigned and unverifiable, the unsecure
badges disappear, and a hosted server relays player messages without anything
to report — configurable live from Options > Mods.

## Behavior

Horizon implements Minecraft 26.3's secure chat itself: a client that signs in
fetches its profile key pair, sends a `ChatSessionUpdate` with the account's
public key, signs every chat line and every command's message arguments, and
paints trust badges on what it receives; a hosted server verifies the sender's
signature chain and relays signed messages with per-guest numbering. This mod
takes the reporting substrate apart at the seams the engine already has:

- **Disable Signing** (client) — the chat session never exists. The public-key
  packet is never sent, chat lines go out with no signature, and command
  arguments go out as `ChatCommandSigned` with an empty signature list,
  exactly what a sessionless client sends. Any session held before the switch
  is dropped at the next line, command or session attempt.
- **Hide Unsecure Indicators / Hide Modified Indicators** (client) — the red
  "Not Secure" badge and the yellow "Modified" badge no longer paint on player
  chat lines. Validation errors keep their badge.
- **Strip Signatures (Server)** (hosted server) — the hub relays every player
  message unsigned: no receiving client ever holds a verifiable, reportable
  line, and the hub keeps no pending-signature bookkeeping for them.
- **Convert To System Messages** (hosted server, off out of the box) — player
  messages reach each guest as a system line (`chat.type.text` with the
  sender's display name and content) instead of a player chat, so clients that
  would badge an unsigned line have nothing signed-shaped to badge.
- **Enabled** — the master switch; off, the engine runs exactly as vanilla.

Settings are read fresh at every gate, so edits apply live. A server with
`enforces_secure_chat` (its own `server.toml` policy) still refuses unsigned
lines and unsigned commands — a mod cannot void the server's own setting, the
same as the original mod on a vanilla enforcing server.

## Scope and provenance

This is an **independent clean-room reimplementation** for Horizon: written
against Horizon's own chat paths from the two originals' public feature lists
only; no code from either project was consulted, copied or ported. It contains
no original code.

- [No Chat Reports](https://modrinth.com/mod/no-chat-reports) by Aizistral
  ([source](https://github.com/Aizistral-Studios/No-Chat-Reports)) — WTFPL,
  reimplemented under MIT.
- [No Chat Restrictions](https://modrinth.com/mod/no-chat-restrictions) by
  Aizistral ([source](https://github.com/Aizistral-Studios/No-Chat-Restrictions))
  — WTFPL, reimplemented under MIT.

### Feature mapping

| Upstream feature | This mod on Horizon |
|---|---|
| Strip signatures from outgoing messages | **Disable Signing**: no session, unsigned lines and command arguments |
| Refuse to send the account's public key | **Disable Signing**: the `ChatSessionUpdate` packet is never sent |
| Server strips signatures before relaying | **Strip Signatures (Server)**: the relay takes every message unsigned |
| `convertToGameMessage` (server) | **Convert To System Messages** (off by default, as upstream's) |
| `hideInsecureMessageIndicators` | **Hide Unsecure Indicators** |
| `hideModifiedMessageIndicators` | **Hide Modified Indicators** |
| `enableMod` | **Enabled** |
| Chat reporting disabled | Falls out of the above: no verifiable message exists anywhere in the chain, and Horizon has no reporting UI or Mojang services surface to feed |
| `demandOnServer` (client refuses servers without the mod) | N/A — Horizon has no client/server protocol that advertises native mods |
| `demandOnClient` (server demands the mod) | N/A — same, from the server side |
| `showServerSafety`, `verifiedIcon`, `whitelistedServers`, `hideWarningToast`, `showNCRButton`, `showReloadButton` | N/A — Horizon has no unsecure-server warning screens or toasts, no server-list mod icons, and no NCR button; there is no signing check to warn about |
| `defaultSigningMode` (Always/Never/Prompt/On Demand) | N/A — Horizon has no per-server signing prompt; Disable Signing is global and live |
| `hideSigningRequestMessage` ("Chat Signing Requested" screen) | N/A — Horizon has no such screen; unattended commands with signable arguments use the engine's own confirm screen, untouched |
| `disableTelemetry`, `removeTelemetryButton` | N/A — Horizon's telemetry is opt-in and off until the player consents; nothing to disable |
| `addQueryData` (server-list ping) | N/A — no such query surface |
| Chat Encryption (`NCR-Encryption.json`) | N/A — the vanilla-server trick of smuggling encrypted payloads through system messages has no Horizon counterpart; not invented here |
| No Chat Restrictions: account-flag bans of the multiplayer menu and chat | N/A — Horizon is an offline rewrite: no Mojang account flags exist and nobody is blocked (the engine's own words in `social.rs`), so the restriction does not exist |
| No Chat Restrictions: the restrictions it reverts on 1.16.4+ clients | N/A — same reason |
| Horizon's own chat "restriction": the hub chat filter (slur list, mutes, tempbans) | N/A by choice — it is server-operator moderation with its own config switch (`server.toml` `[chat_filter] enabled`), so a bypass would duplicate an existing setting |

The "Only Show Secure Chat" option (Horizon's vanilla-equivalent
`onlyShowSecureChat`) is the player's own preference and stays untouched.

## Configuration

Horizon stores mod settings as files:
`<game dir>/horizon-mods/.horizon-mod-data/no_chat_reports/<key>` — one value
per file, edited by hand or through the mod's settings page, applied live.
Missing files mean the defaults (everything on except the conversion). The
keys: `enabled`, `disable_signing`, `hide_unsecure_indicators`,
`hide_modified_indicators`, `strip_relayed_signatures`,
`convert_to_system_messages` — a file holding `false` turns that switch off,
anything else (or a missing file) keeps its default.

## Building

The `.hnmod` is source-only: Horizon's launcher compiles it against the pinned
Horizon revision on install.

```sh
python3 -I tools/horizon.py check   # stage the pinned source, apply the mixins, type-check
python3 -I tools/horizon.py test    # the runtime's tests in the staged client lib
python3 -I tools/package.py         # target/no_chat_reports-<version>.hnmod
```

## License

MIT — see [LICENSE](LICENSE). The two upstream mods are WTFPL; their names and
links are kept here as the feature ideas' origin.

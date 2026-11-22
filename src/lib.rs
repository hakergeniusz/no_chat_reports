//! No Chat Reports, the native-route native mod.
//!
//! The brain is pure standard library — no game types — so it compiles and
//! tests wherever. Six switches on one decision table. On a client, "Disable
//! Signing" keeps the chat session from ever existing: the account's public
//! key packet is never sent and every chat line and command argument goes
//! out unsigned, so no message can be attributed to the account, let alone
//! reported. The two indicator switches drop the unsecure and modified
//! badges the client paints on unsigned lines. On a hosted server, "Strip
//! Signatures" relays every player message as unsigned (the reporting chain
//! never forms) and "Convert To System Messages" turns them into system
//! lines for the clients that would badge them.
//!
//! Every switch is a registered setting, stored as usual under
//! `horizon-mods/.horizon-mod-data/no_chat_reports/` — one file per key —
//! which every gate reads fresh off the disk, the same path the settings
//! page writes, so the game-crate mixins and this crate meet at one store.
//!
//! Clean-room: written for Horizon from Horizon's own chat paths and the two
//! original mods' public feature lists; it copies nothing from any mod.
//!
//! Upstream: No Chat Reports (Aizistral-Studios, WTFPL) and No Chat
//! Restrictions (Aizistral-Studios, WTFPL), reimplemented under MIT without
//! their code.
//!
//! Copyright (c) 2026 hakergeniusz. MIT.

use std::path::PathBuf;

use mod_api::{Hooks, RegistrationContext, Registrar, SettingSpec};

/// The mod's registry id: the settings folder and the Options > Mods row.
pub const ID: &str = "no_chat_reports";

/// The master switch: everything else answers to it.
pub const ENABLED: &str = "enabled";
/// The client's signing kill: no chat session, no public key packet, unsigned
/// chat lines and command arguments (upstream's core behavior).
pub const DISABLE_SIGNING: &str = "disable_signing";
/// The client stops painting the red "Not Secure" badge on unsigned lines
/// (upstream hideInsecureMessageIndicators).
pub const HIDE_UNSECURE: &str = "hide_unsecure_indicators";
/// The client stops painting the yellow "Modified" badge on changed lines
/// (upstream hideModifiedMessageIndicators).
pub const HIDE_MODIFIED: &str = "hide_modified_indicators";
/// The hosted server relays every player message unsigned, so the reporting
/// chain never forms for any receiver (upstream's server-side stripping).
pub const STRIP_RELAYED: &str = "strip_relayed_signatures";
/// The hosted server turns player messages into system lines, so even a
/// signing-enforcing client has nothing signed to show or report (upstream
/// convertToGameMessage). Off out of the box, like upstream's.
pub const CONVERT_SYSTEM: &str = "convert_to_system_messages";

/// One settings row: the key, its translation key, and what a missing file
/// means. Every switch defaults on except the conversion, which changes what
/// other players' clients receive and stays opt-in, as upstream's does.
pub const ROWS: [(&str, &str, bool); 6] = [
    (ENABLED, "no_chat_reports.setting.enabled", true),
    (DISABLE_SIGNING, "no_chat_reports.setting.disable_signing", true),
    (HIDE_UNSECURE, "no_chat_reports.setting.hide_unsecure", true),
    (HIDE_MODIFIED, "no_chat_reports.setting.hide_modified", true),
    (STRIP_RELAYED, "no_chat_reports.setting.strip_relayed", true),
    (CONVERT_SYSTEM, "no_chat_reports.setting.convert_system", false),
];

/// The six switches, as every gate reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Switches {
    pub enabled: bool,
    pub disable_signing: bool,
    pub hide_unsecure_indicators: bool,
    pub hide_modified_indicators: bool,
    pub strip_relayed_signatures: bool,
    pub convert_to_system_messages: bool,
}

/// One switch out of its file: a missing file keeps the row's default, an
/// explicit `false` turns it off, and anything else is on — trimmed like the
/// game reads its settings.
pub fn switch_on(read: impl FnOnce(&str) -> Option<String>, key: &str, default: bool) -> bool {
    match read(key) {
        None => default,
        Some(v) => !(v.trim() == "false"),
    }
}

/// The switches out of the settings folder's files.
pub fn parse(get: impl Fn(&str) -> Option<String>) -> Switches {
    Switches {
        enabled: switch_on(&get, ENABLED, true),
        disable_signing: switch_on(&get, DISABLE_SIGNING, true),
        hide_unsecure_indicators: switch_on(&get, HIDE_UNSECURE, true),
        hide_modified_indicators: switch_on(&get, HIDE_MODIFIED, true),
        strip_relayed_signatures: switch_on(&get, STRIP_RELAYED, true),
        convert_to_system_messages: switch_on(&get, CONVERT_SYSTEM, false),
    }
}

/// The switches off this game's disk. The folder is the game's own settings
/// store (`horizon-mods/.horizon-mod-data/<mod id>/`, one file per key) —
/// what the settings page writes and what every gate reads meet at one path,
/// on the window's game and on a dedicated server's thread alike.
pub fn switches() -> Switches {
    parse(|key| std::fs::read_to_string(dir().join(key)).ok())
}

/// The mod's settings folder on this game's disk.
fn dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_default()
        .join("horizon-mods")
        .join(".horizon-mod-data")
        .join(ID)
}

/// The client kills the chat session: the account's public key is never sent,
/// and with no session the client cannot sign a line or a command argument.
pub fn signing_blocked(sw: &Switches) -> bool {
    sw.enabled && sw.disable_signing
}

/// The client drops the "Not Secure" badge.
pub fn unsecure_badge_hidden(sw: &Switches) -> bool {
    sw.enabled && sw.hide_unsecure_indicators
}

/// The client drops the "Modified" badge.
pub fn modified_badge_hidden(sw: &Switches) -> bool {
    sw.enabled && sw.hide_modified_indicators
}

/// The hosted server relays player messages unsigned.
pub fn relay_stripped(sw: &Switches) -> bool {
    sw.enabled && sw.strip_relayed_signatures
}

/// The hosted server turns player messages into system lines.
pub fn relay_converted(sw: &Switches) -> bool {
    sw.enabled && sw.convert_to_system_messages
}

#[derive(Default)]
pub struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {
    /// The six switches on the mod's page, in their defaults.
    fn register(&self, registrar: &mut dyn Registrar, _: &RegistrationContext) {
        for (key, label, default) in ROWS {
            let _ = registrar.register_setting(SettingSpec::toggle(key, label, default));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Files for one test: the key that is set, and everything else missing.
    fn files<'a>(present: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |key| {
            present
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, v)| (*v).to_owned())
        }
    }

    /// One switch's file: missing keeps the default, an explicit `false` is
    /// off, trimmed anything else is on.
    #[test]
    fn files_mean_defaults_until_edited() {
        let one = |key: &str| (key == ENABLED).then(|| " true \n".to_owned());
        assert!(switch_on(|_| None, DISABLE_SIGNING, true), "missing is on");
        assert!(!switch_on(|_| None, CONVERT_SYSTEM, false), "missing is off");
        assert!(switch_on(one, DISABLE_SIGNING, true), "anything but false is on");
        assert!(!switch_on(
            |key| (key == DISABLE_SIGNING).then(|| "false".to_owned()),
            DISABLE_SIGNING,
            true
        ));
    }

    /// The whole table parses out of the folder's files.
    #[test]
    fn the_table_parses_out_of_the_files() {
        let sw = parse(files(&[
            (ENABLED, "true"),
            (DISABLE_SIGNING, "false"),
            (CONVERT_SYSTEM, "true"),
        ]));
        assert!(sw.enabled);
        assert!(!sw.disable_signing, "an explicit false turns it off");
        assert!(sw.hide_unsecure_indicators, "missing keeps the default");
        assert!(sw.hide_modified_indicators, "missing keeps the default");
        assert!(sw.strip_relayed_signatures, "missing keeps the default");
        assert!(sw.convert_to_system_messages, "an explicit true turns it on");
        assert!(!signing_blocked(&sw), "the master is on but signing is off");
    }

    /// The gates answer only when the master is on.
    #[test]
    fn the_gates_answer_to_the_master() {
        let all_on = parse(|_| Some("true".to_owned()));
        assert!(signing_blocked(&all_on));
        assert!(unsecure_badge_hidden(&all_on));
        assert!(modified_badge_hidden(&all_on));
        assert!(relay_stripped(&all_on));
        assert!(relay_converted(&all_on), "an explicit true turns the opt-in on");
        let master_off = parse(files(&[(ENABLED, "false")]));
        assert!(!signing_blocked(&master_off));
        assert!(!unsecure_badge_hidden(&master_off));
        assert!(!modified_badge_hidden(&master_off));
        assert!(!relay_stripped(&master_off));
        assert!(!relay_converted(&master_off));
    }

    /// The six registered rows carry the mod's keys, labels and defaults, in
    /// order — what the settings page shows.
    #[test]
    fn the_rows_are_the_six_switches() {
        let keys: Vec<&str> = ROWS.iter().map(|(k, _, _)| *k).collect();
        assert_eq!(
            keys,
            vec![
                ENABLED,
                DISABLE_SIGNING,
                HIDE_UNSECURE,
                HIDE_MODIFIED,
                STRIP_RELAYED,
                CONVERT_SYSTEM
            ]
        );
        assert_eq!(ROWS[5].2, false, "the conversion stays opt-in");
    }
}

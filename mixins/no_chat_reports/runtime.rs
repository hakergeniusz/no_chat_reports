// No Chat Reports' brain: pure standard library, no game types, so it compiles and
// tests wherever — as this crate's library and, injected, as a module of the LAN
// hub. Six switches on one decision table. On a client, "Disable Signing" keeps
// the chat session from ever existing: the account's public key packet is never
// sent and every chat line and command argument goes out unsigned, so no message
// can be attributed to the account, let alone reported. The two indicator rows
// drop the unsecure and modified badges the client paints on unsigned lines. On
// a hosted server, "Strip Signatures" relays every player message as unsigned
// (the reporting chain never forms) and "Convert To System Messages" turns them
// into system lines for the clients that would badge them. Every switch is a
// file under the mod's data folder, read fresh at every gate.
//
// Clean-room: written for Horizon from Horizon's own chat paths and the two
// original mods' public feature lists; it copies nothing from any mod.
//
// Upstream: No Chat Reports (Aizistral-Studios, WTFPL) and No Chat Restrictions
// (Aizistral-Studios, WTFPL), reimplemented under MIT without their code.
//
// Copyright (c) 2026 hakergeniusz. MIT.

/// The mod's registry id: the settings folder and the Options > Mods row.
pub const ID: &str = "no_chat_reports";
/// The version the row shows.
pub const VERSION: &str = "0.1.0";
/// The mod's description for its row.
pub const DESCRIPTION: &str = "Unsigned, unreportable chat: no chat session or public \
key leaves the client, a hosted server relays player messages unsigned or as system \
lines, and the unsecure badges disappear. Covers No Chat Reports and No Chat \
Restrictions (both WTFPL, reimplemented under MIT).";

/// The master switch's setting file: everything else answers to it.
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

/// One settings row: the file name, the label on the page, and what a missing
/// file means. Every switch defaults on except the conversion, which changes
/// what other players' clients receive and stays opt-in, as upstream's does.
pub const ROWS: [(&str, &str, bool); 6] = [
    (ENABLED, "Enabled", true),
    (DISABLE_SIGNING, "Disable Signing", true),
    (HIDE_UNSECURE, "Hide Unsecure Indicators", true),
    (HIDE_MODIFIED, "Hide Modified Indicators", true),
    (STRIP_RELAYED, "Strip Signatures (Server)", true),
    (CONVERT_SYSTEM, "Convert To System Messages", false),
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

/// Whether row `key` is on in `sw`.
fn row_on(sw: &Switches, key: &str) -> bool {
    match key {
        ENABLED => sw.enabled,
        DISABLE_SIGNING => sw.disable_signing,
        HIDE_UNSECURE => sw.hide_unsecure_indicators,
        HIDE_MODIFIED => sw.hide_modified_indicators,
        STRIP_RELAYED => sw.strip_relayed_signatures,
        _ => sw.convert_to_system_messages,
    }
}

/// The settings page's row `i`: the label in the state the files hold.
pub fn label(i: usize, sw: &Switches) -> Option<String> {
    let (key, name, _) = ROWS.get(i)?;
    let on = row_on(sw, key);
    Some(format!("{name}: {}", if on { "ON" } else { "OFF" }))
}

/// A click on the settings page's row `i`: the row flips from the state the
/// switches hold, the rest are nothing (the page has six rows). Returns the
/// file to write and its new value, so the caller stores only real clicks.
pub fn click(i: usize, sw: &Switches) -> Option<(&'static str, bool)> {
    let (key, _, _) = ROWS.get(i)?;
    Some((key, !row_on(sw, key)))
}

/// What to store for a value: the setting key and its file text.
pub fn stored(key: &str, on: bool) -> (&'static str, String) {
    let key = ROWS
        .iter()
        .find(|(name, _, _)| *name == key)
        .map_or(ENABLED, |(name, _, _)| name);
    (key, if on { "true" } else { "false" }.to_owned())
}

/// The switches out of the settings files on this game's disk. The folder is
/// the game's own `Game::native_data_dir` — `game_dir()` is the working
/// directory, `horizon-mods` is `mod_list::DIR` and `.horizon-mod-data` the
/// engine's own literal — so what the settings page writes and what every
/// gate reads meet at one path, on the window's game and on a dedicated
/// server's thread alike.
pub fn switches() -> Switches {
    parse(|key| std::fs::read_to_string(dir().join(key)).ok())
}

/// The mod's settings folder on this game's disk. The two literals are the
/// game's own (`mod_list::DIR`, unreachable from this standalone build, and
/// the `.horizon-mod-data` name inside `native_data_dir`); the pinned
/// Horizon revision is what this module is checked against.
fn dir() -> std::path::PathBuf {
    std::env::current_dir()
        .unwrap_or_default()
        .join("horizon-mods")
        .join(".horizon-mod-data")
        .join(ID)
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
    /// off, trimmed anything else is on — and what a click writes reads back
    /// as that state.
    #[test]
    fn files_mean_defaults_until_edited() {
        let one = |key: &str| (key == ENABLED).then(|| " true \n".to_owned());
        assert!(switch_on(|_| None, DISABLE_SIGNING, true), "missing is on");
        assert!(
            !switch_on(|_| None, CONVERT_SYSTEM, false),
            "missing is off"
        );
        assert!(
            switch_on(one, DISABLE_SIGNING, true),
            "anything but false is on"
        );
        assert!(!switch_on(
            |key| (key == DISABLE_SIGNING).then(|| "false".to_owned()),
            DISABLE_SIGNING,
            true
        ));
        for on in [true, false] {
            let (key, value) = stored(DISABLE_SIGNING, on);
            assert_eq!(key, DISABLE_SIGNING);
            let read = |k: &str| (k == key).then(|| value.clone());
            assert_eq!(switch_on(read, key, !on), on, "{value} reads back {on}");
        }
    }

    /// The whole table out of the six files: each default stands until its own
    /// file says otherwise, and the master switch turns every gate off at once.
    #[test]
    fn the_table_parses_and_gates() {
        let all = parse(files(&[]));
        assert!(all.enabled);
        assert!(all.disable_signing);
        assert!(all.hide_unsecure_indicators);
        assert!(all.hide_modified_indicators);
        assert!(all.strip_relayed_signatures);
        assert!(!all.convert_to_system_messages);
        for (key, _, _) in ROWS {
            let sw = parse(files(&[(key, "false")]));
            assert!(!row_on(&sw, key), "{key} off");
        }
        let master_off = parse(files(&[(ENABLED, "false")]));
        assert!(!signing_blocked(&master_off), "the master silences all");
        assert!(!unsecure_badge_hidden(&master_off));
        assert!(!modified_badge_hidden(&master_off));
        assert!(!relay_stripped(&master_off));
        assert!(!relay_converted(&master_off));
    }

    /// Each gate is its master switch AND its row: the row alone, or the
    /// master alone, never acts.
    #[test]
    fn every_gate_answers_to_both_switches() {
        let off_row = parse(files(&[(DISABLE_SIGNING, "false")]));
        assert!(!signing_blocked(&off_row));
        let off_row = parse(files(&[(HIDE_UNSECURE, "false")]));
        assert!(!unsecure_badge_hidden(&off_row));
        let off_row = parse(files(&[(HIDE_MODIFIED, "false")]));
        assert!(!modified_badge_hidden(&off_row));
        let off_row = parse(files(&[(STRIP_RELAYED, "false")]));
        assert!(!relay_stripped(&off_row));
        let off_row = parse(files(&[(CONVERT_SYSTEM, "true")]));
        assert!(relay_converted(&off_row), "the row's own default is off");
        let all = parse(files(&[]));
        assert!(signing_blocked(&all));
        assert!(unsecure_badge_hidden(&all));
        assert!(modified_badge_hidden(&all));
        assert!(relay_stripped(&all));
        assert!(!relay_converted(&all));
    }

    /// The settings page shows the six rows in the state the files hold, and
    /// a click flips exactly that row; clicks past the page are nothing.
    #[test]
    fn the_page_shows_and_flips_the_six_rows() {
        let sw = parse(files(&[]));
        let names: Vec<String> = (0..6).filter_map(|i| label(i, &sw)).collect();
        assert_eq!(
            names,
            [
                "Enabled: ON",
                "Disable Signing: ON",
                "Hide Unsecure Indicators: ON",
                "Hide Modified Indicators: ON",
                "Strip Signatures (Server): ON",
                "Convert To System Messages: OFF",
            ]
        );
        assert!(label(6, &sw).is_none(), "the page has six rows");
        let (key, new) = click(5, &sw).expect("a real row");
        assert_eq!((key, new), (CONVERT_SYSTEM, true));
        let sw = parse(files(&[(CONVERT_SYSTEM, "true")]));
        let (key, new) = click(5, &sw).expect("a real row");
        assert_eq!((key, new), (CONVERT_SYSTEM, false));
        assert!(click(9, &sw).is_none(), "past the page is nothing");
    }

    /// Stored files round-trip: only a real row's key is written, and reading
    /// it back gives the state the click asked for.
    #[test]
    fn stored_files_round_trip() {
        let sw = parse(files(&[]));
        let (key, value) = click(0, &sw).map(|(k, v)| stored(k, v)).expect("row 0");
        assert_eq!((key, value.as_str()), (ENABLED, "false"));
        let sw = parse(files(&[(ENABLED, "false")]));
        let (key, value) = click(0, &sw).map(|(k, v)| stored(k, v)).expect("row 0");
        assert_eq!((key, value.as_str()), (ENABLED, "true"));
        let (key, _) = stored("not-a-row", true);
        assert_eq!(key, ENABLED, "an unknown key lands on the master");
    }
}

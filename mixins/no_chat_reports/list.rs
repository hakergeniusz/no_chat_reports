// The mod's row in Options > Mods: no keybinds — the mod answers to its
// switches, not to a key.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::native_hooks::NativeMod;
    use horizon_client::lan::hub::no_chat_reports as ncr;
    mods.push(NativeMod {
        id: ncr::ID.into(),
        title: format!("No Chat Reports {}", ncr::VERSION),
        description: ncr::DESCRIPTION.into(),
        keys: vec![],
    });
}

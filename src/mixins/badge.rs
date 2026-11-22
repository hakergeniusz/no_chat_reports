// The trust badge's gate: the tag a player's chat line carries. While Hide
// Unsecure Indicators is on, the "Not Secure" badge (the engine's system
// color, no icon) is dropped; while Hide Modified Indicators is on, the
// "Modified" badge (the one with the icon) is dropped. Validation errors
// keep their red badge, and a signed-and-intact line never had one.
// Copyright (c) 2026 hakergeniusz. MIT.
fn no_chat_reports_badge(
    value: Option<horizon_client::chat::Tag>,
) -> Option<horizon_client::chat::Tag> {
    let sw = no_chat_reports::switches();
    if let Some(tag) = &value {
        if tag.icon && no_chat_reports::modified_badge_hidden(&sw) {
            return None;
        }
        if !tag.icon
            && tag.color == horizon_client::chat::SYSTEM_INDICATOR
            && no_chat_reports::unsecure_badge_hidden(&sw)
        {
            return None;
        }
    }
    value
}

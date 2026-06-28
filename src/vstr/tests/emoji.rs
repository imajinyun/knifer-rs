use crate::vstr::*;

#[test]
fn emoji_helpers_detect_and_remove_common_sequences() {
    assert!(contains_emoji("ship it 🚀"));
    assert!(contains_emoji("go ✅"));
    assert!(contains_emoji("flag 🇨🇳"));
    assert!(contains_emoji("key 1️⃣"));
    assert!(!contains_emoji("knifer-rs 123"));
    assert_eq!(remove_emoji("ship 🚀 now"), "ship  now");
    assert_eq!(remove_emoji("ok ✅"), "ok ");
    assert_eq!(remove_emoji("key 1️⃣ done"), "key  done");
    assert_eq!(remove_emoji("knifer-rs 123"), "knifer-rs 123");
}

#[test]
fn emoji_options_customize_matching_and_replacement() {
    let matcher = with_emoji_matcher(|input| input.contains(":rocket:"));
    assert!(contains_emoji_with_options("ship :rocket:", &matcher));
    assert!(!contains_emoji_with_options("ship 🚀", &matcher));

    let replacer = with_emoji_replacer(|input| input.replace(":rocket:", ""));
    assert_eq!(
        remove_emoji_with_options("ship :rocket: now", &replacer),
        "ship  now"
    );
    assert_eq!(remove_emoji_with_options("ship 🚀", &replacer), "ship 🚀");

    let defaults = EmojiOptions::new();
    assert!(contains_emoji_with_options("ship 🚀", &defaults));
    assert_eq!(remove_emoji_with_options("ship 🚀", &defaults), "ship ");
}

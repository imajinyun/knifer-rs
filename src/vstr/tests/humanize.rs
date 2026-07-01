use crate::vstr::*;

#[test]
fn pluralize_follows_regular_and_irregular_english_rules() {
    assert_eq!(pluralize("cat", 1), "cat");
    assert_eq!(pluralize("cat", -1), "cat");
    assert_eq!(pluralize("cat", 0), "cats");
    assert_eq!(pluralize("cat", 2), "cats");
    assert_eq!(pluralize("bus", 3), "buses");
    assert_eq!(pluralize("box", 2), "boxes");
    assert_eq!(pluralize("dish", 2), "dishes");
    assert_eq!(pluralize("church", 2), "churches");
    assert_eq!(pluralize("city", 5), "cities");
    assert_eq!(pluralize("day", 2), "days");
    assert_eq!(pluralize("person", 4), "people");
    assert_eq!(pluralize("child", 2), "children");
    assert_eq!(pluralize("mouse", 2), "mice");
    assert_eq!(pluralize("", 2), "");
}

#[test]
fn ordinalize_uses_standard_english_suffix_rules() {
    assert_eq!(ordinalize(0), "0th");
    assert_eq!(ordinalize(1), "1st");
    assert_eq!(ordinalize(2), "2nd");
    assert_eq!(ordinalize(3), "3rd");
    assert_eq!(ordinalize(4), "4th");
    assert_eq!(ordinalize(11), "11th");
    assert_eq!(ordinalize(12), "12th");
    assert_eq!(ordinalize(13), "13th");
    assert_eq!(ordinalize(21), "21st");
    assert_eq!(ordinalize(22), "22nd");
    assert_eq!(ordinalize(23), "23rd");
    assert_eq!(ordinalize(111), "111th");
    assert_eq!(ordinalize(101), "101st");
    assert_eq!(ordinalize(-1), "-1st");
    assert_eq!(ordinalize(-13), "-13th");
}

#[test]
fn number_format_groups_thousands_without_locale_data() {
    assert_eq!(number_format(0), "0");
    assert_eq!(number_format(1), "1");
    assert_eq!(number_format(12), "12");
    assert_eq!(number_format(123), "123");
    assert_eq!(number_format(1_234), "1,234");
    assert_eq!(number_format(12_345), "12,345");
    assert_eq!(number_format(1_234_567), "1,234,567");
    assert_eq!(number_format(-1_234), "-1,234");
    assert_eq!(number_format(-12_345), "-12,345");
    assert_eq!(number_format(i64::MAX), "9,223,372,036,854,775,807");
}

#[test]
fn human_bytes_scales_with_binary_iec_units() {
    assert_eq!(human_bytes(0), "0 B");
    assert_eq!(human_bytes(512), "512 B");
    assert_eq!(human_bytes(1023), "1023 B");
    assert_eq!(human_bytes(1024), "1 KiB");
    assert_eq!(human_bytes(1536), "1.5 KiB");
    assert_eq!(human_bytes(1_048_576), "1 MiB");
    assert_eq!(human_bytes(5_242_880), "5 MiB");
    assert_eq!(human_bytes(1_073_741_824), "1 GiB");
    // Round-half-up on the single decimal digit stays deterministic.
    assert_eq!(human_bytes(1_127), "1.1 KiB");
}

#[test]
fn human_duration_emits_largest_nonzero_units() {
    use std::time::Duration;

    assert_eq!(human_duration(Duration::from_secs(0)), "0s");
    assert_eq!(human_duration(Duration::from_secs(1)), "1s");
    assert_eq!(human_duration(Duration::from_secs(59)), "59s");
    assert_eq!(human_duration(Duration::from_secs(60)), "1m");
    assert_eq!(human_duration(Duration::from_secs(90)), "1m 30s");
    assert_eq!(human_duration(Duration::from_secs(3_600)), "1h");
    assert_eq!(human_duration(Duration::from_secs(3_661)), "1h 1m 1s");
    assert_eq!(human_duration(Duration::from_secs(90_061)), "1d 1h 1m 1s");
    assert_eq!(human_duration(Duration::from_millis(500)), "500ms");
    assert_eq!(human_duration(Duration::from_micros(250)), "250\u{00b5}s");
    assert_eq!(human_duration(Duration::from_nanos(7)), "7ns");
}

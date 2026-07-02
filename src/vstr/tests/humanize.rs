use crate::vstr::*;

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

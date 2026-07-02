//! Byte and encoding facades for non-UTF-8 and BOM boundaries.

use kniferrs::vbytes;
use kniferrs::vencoding::{self, Bom};

fn main() {
    let bytes = [b'a', 0xff, b'b'];
    assert_eq!(vbytes::byte_len(&bytes), 3);
    assert!(!vbytes::is_utf8(&bytes));
    assert_eq!(vbytes::sub(&bytes, 1, 2), &[0xff]);
    assert_eq!(vbytes::find_all(b"aaaa", b"aa"), vec![(0, 2), (2, 4)]);
    assert_eq!(vbytes::replace_all(&bytes, &[0xff], b"?"), b"a?b");

    let encoded = [0xEF, 0xBB, 0xBF, b'a', 0xff];
    assert_eq!(vencoding::detect_bom(&encoded), Some(Bom::Utf8));
    assert_eq!(vencoding::strip_bom(&encoded), &[b'a', 0xff]);
    assert!(vencoding::validate_utf8_without_bom(&encoded).is_err());
    assert_eq!(
        vencoding::decode_utf8_lossy_without_bom(&encoded),
        "a\u{FFFD}"
    );
}

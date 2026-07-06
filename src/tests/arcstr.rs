#![allow(clippy::missing_panics_doc)]

use arcstr::ArcStr;

use crate::{ArcStrUtf8Encoder, Utf8Encoder};

#[test]
pub fn valid_utf8_roundtrips() {
    let str = "Příliš žluťoučký kůň úpěl ďábelské ódy.";

    assert_eq!(str.as_bytes().to_utf8_arcstr(), str);
}

#[test]
pub fn invalid_bytes_map_through_chars() {
    let bytes = b"Hello\xFFWorld";

    assert_eq!(bytes.to_utf8_arcstr(), "Hello˙World");
}

#[test]
pub fn empty_input_yields_empty_arcstr() {
    let bytes: &[u8] = b"";

    assert_eq!(bytes.to_utf8_arcstr(), ArcStr::new());
}

#[test]
pub fn arcstr_is_covered_by_utf8_encoder() {
    let str = ArcStr::from("Příliš žluťoučký kůň úpěl ďábelské ódy.");

    assert_eq!(str.to_utf8_string(), str.as_str());
    assert_eq!(str.to_utf8_arcstr(), str);
}

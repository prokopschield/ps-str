#![allow(clippy::expect_used)]
#![allow(clippy::missing_panics_doc)]

use encoding::Encoding;

use crate::*;

#[test]
pub fn zlutoucky_kun() {
    let str = "Příliš žluťoučký kůň úpěl ďábelské ódy.";

    let utf8 = b"P\xc5\x99\xc3\xadli\xc5\xa1 \xc5\xbelu\xc5\xa5ou\xc4\x8dk\xc3\xbd k\xc5\xaf\xc5\x88 \xc3\xbap\xc4\x9bl \xc4\x8f\xc3\xa1belsk\xc3\xa9 \xc3\xb3dy.";

    assert_eq!(str.as_bytes(), utf8, "utf8 does not match str");

    let latin2 = encoding::all::ISO_8859_2
        .encode(str, encoding::EncoderTrap::Strict)
        .expect("Failed to encode str as latin2");

    assert_eq!(
        latin2.to_utf8_string(),
        utf8.to_utf8_string(),
        "latin2 does not match utf8"
    );

    let win1250 = encoding::all::WINDOWS_1250
        .encode(str, encoding::EncoderTrap::Strict)
        .expect("Failed to encode str as win1250");

    assert_ne!(latin2, win1250, "latin2 should not match win1250");

    assert_eq!(
        latin2.to_utf8_string(),
        win1250.to_utf8_string(),
        "latin2 does not match win1250"
    );
}


use crate::*;

#[test]
pub fn zlutoucky_kun() {
    let bytes = b"P\xc5\x99\xc3\xadli\xc5\xa1 \xc5\xbelu\xc5\xa5ou\xc4\x8dk\xc3\xbd k\xc5\xaf\xc5\x88 \xc3\xbap\xc4\x9bl \xc4\x8f\xc3\xa1belsk\xc3\xa9 \xc3\xb3dy.";
    let str = "Příliš žluťoučký kůň úpěl ďábelské ódy.";

    assert_eq!(bytes.to_utf8_string(), str, "Decoding failed");
}

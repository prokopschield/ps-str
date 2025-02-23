#![allow(clippy::expect_used)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::{fs::OpenOptions, io::Write};

use encoding::{
    all::{ISO_8859_1, ISO_8859_2, WINDOWS_1250, WINDOWS_1252},
    Encoding,
};

use crate::Utf8Encoder;

#[test]
pub fn char_table() -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("chars")?;

    writeln!(file, "0x00 CAN LA1 LA2 W52 W50 L1 L2 W2 W0")?;

    let fmt = |str: &str| {
        let fmt = format!("{str:?}");

        if fmt.chars().count() > 3 {
            let chars: Vec<char> = str.chars().collect();

            format!("{}", chars[0] as u8)
        } else {
            fmt
        }
    };

    let checkmark = |condition: bool| {
        if condition {
            "✅"
        } else {
            "❌"
        }
    };

    let invalid = |s: &str| {
        let chars: Vec<char> = s.chars().collect();
        let c = chars[0];

        let valid = ['\\', '\"', '', ' ', '\u{AD}'];

        c >= ' ' && !valid.contains(&c) && format!("{s:?}").chars().count() > 3
    };

    for i in u8::MIN..=u8::MAX {
        let raw = [i];

        let canonical = raw.to_utf8_string();

        let latin1 = ISO_8859_1
            .decode(&raw, encoding::DecoderTrap::Replace)
            .expect("Failed decoding ISO_8859_1");

        let latin2 = ISO_8859_2
            .decode(&raw, encoding::DecoderTrap::Replace)
            .expect("Failed decoding ISO_8859_2");

        let win1252 = WINDOWS_1252
            .decode(&raw, encoding::DecoderTrap::Replace)
            .expect("Failed decoding WINDOWS_1252");

        let win1250 = WINDOWS_1250
            .decode(&raw, encoding::DecoderTrap::Replace)
            .expect("Failed decoding WINDOWS_1250");

        if !invalid(&canonical)
            && canonical == latin1
            && canonical == latin2
            && canonical == win1252
            && canonical == win1250
        {
            continue;
        }

        writeln!(
            file,
            "0x{:2x} {} {} {} {} {} {} {} {} {}",
            i,
            fmt(&canonical),
            fmt(&latin1),
            fmt(&latin2),
            fmt(&win1252),
            fmt(&win1250),
            checkmark(canonical == latin1),
            checkmark(canonical == latin2),
            checkmark(canonical == win1252),
            checkmark(canonical == win1250)
        )?;

        assert!(
            !invalid(&canonical),
            "Disallowed character {} '{}'",
            fmt(&canonical),
            canonical
        );

        if canonical == latin2 {
            continue;
        }

        assert!(invalid(&latin2), "{i} should be {latin2}, is {canonical}");

        if canonical == win1250 {
            continue;
        }

        assert!(invalid(&win1250), "{i} should be {win1250}, is {canonical}");

        if canonical == win1252 {
            continue;
        }

        assert!(invalid(&win1252), "{i} should be {win1252}, is {canonical}");

        if canonical == latin1 {
            continue;
        }

        assert!(invalid(&latin1), "{i} should be {latin1}, is {canonical}");

        assert!(canonical == "�", "Character {canonical} not match encoding");
    }

    Ok(())
}

use std::path::PathBuf;

use crate::PathUtf8Encoder;

impl PathUtf8Encoder for PathBuf {
    fn to_utf8_string(&self) -> String {
        self.as_path().to_utf8_string()
    }
}

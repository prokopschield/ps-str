use std::path::Path;

use crate::{PathUtf8Encoder, Utf8Encoder};

impl PathUtf8Encoder for Path {
    fn to_utf8_string(&self) -> String {
        self.as_os_str().as_encoded_bytes().to_utf8_string()
    }
}

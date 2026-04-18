pub mod chars;
pub mod path;

use std::path::PathBuf;

pub use chars::CHARS;
pub use path::PathUtf8Encoder;

#[cfg(test)]
pub mod tests;

pub trait Utf8Encoder {
    fn to_utf8_string(&self) -> String;

    /// Converts a byte sequence into a [`PathBuf`] using the same decoding rules
    /// as [`Self::to_utf8_string`].
    ///
    /// Valid UTF-8 runs pass through unchanged; other bytes are
    /// mapped through the [`CHARS`] fallback table. This is **lossy**
    /// for paths containing non-UTF-8 bytes.
    ///
    /// To preserve the original bytes exactly (e.g. when opening an existing
    /// file with a non-UTF-8 name on Unix), do not use this method. Convert
    /// the bytes directly into an [`OsString`](std::ffi::OsString) via a platform-specific API
    /// such as `std::os::unix::ffi::OsStringExt::from_vec`, then into a
    /// [`PathBuf`].
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use ps_str::Utf8Encoder;
    ///
    /// let bytes = b"hello.txt";
    ///
    /// assert_eq!(bytes.to_utf8_path(), PathBuf::from("hello.txt"));
    /// ```
    fn to_utf8_path(&self) -> PathBuf {
        self.to_utf8_string().into()
    }
}

impl<T: AsRef<[u8]>> Utf8Encoder for T {
    /// Converts a byte sequence into a UTF-8 string, replacing invalid bytes
    /// with fallback characters.
    ///
    /// This method processes bytes sequentially, attempting to parse valid UTF-8
    /// sequences. When valid UTF-8 is encountered, it is appended directly to the
    /// result string. When an invalid UTF-8 sequence is detected, the valid portion
    /// preceding the error is appended, then the offending byte is replaced, and
    /// processing continues from the next byte.
    ///
    /// # Returns
    ///
    /// A `String` containing the decoded content, guaranteed to be valid UTF-8
    /// even if the input contained invalid byte sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_str::Utf8Encoder;
    ///
    /// let bytes = b"Hello\xFFWorld";
    /// let result = bytes.to_utf8_string();
    ///
    /// assert_eq!(result, "Hello˙World");
    /// ```
    fn to_utf8_string(&self) -> String {
        let bytes = self.as_ref();
        let mut result = String::with_capacity(bytes.len());
        let mut i = 0;

        while i < bytes.len() {
            match std::str::from_utf8(&bytes[i..]) {
                Ok(valid_str) => {
                    result.push_str(valid_str);
                    break;
                }
                Err(e) => {
                    if e.valid_up_to() > 0 {
                        let valid_part = &bytes[i..i + e.valid_up_to()];

                        // SAFETY: valid_up_to() guarantees this slice contains only complete, valid UTF-8 characters
                        result.push_str(unsafe { std::str::from_utf8_unchecked(valid_part) });
                        i += e.valid_up_to();
                    }

                    if let Some(c) = bytes.get(i) {
                        result.push(CHARS[*c as usize]);
                        i += 1;
                    }
                }
            }
        }

        result
    }
}

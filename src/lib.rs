pub mod chars;
pub use chars::CHARS;

#[cfg(test)]
pub mod tests;

pub trait Utf8Encoder {
    fn to_utf8_string(&self) -> String;
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

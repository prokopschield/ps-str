mod variants;

use std::path::PathBuf;

pub trait Utf8Encoder {
    /// Converts a byte sequence into a UTF-8 string, replacing invalid bytes
    /// with fallback characters from the [`CHARS`](crate::CHARS) table.
    fn to_utf8_string(&self) -> String;

    /// Converts a byte sequence into a [`PathBuf`] using the same decoding rules
    /// as [`Self::to_utf8_string`].
    ///
    /// Valid UTF-8 runs pass through unchanged; other bytes are
    /// mapped through the [`CHARS`](crate::CHARS) fallback table. This is **lossy**
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

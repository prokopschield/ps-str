use crate::Utf8Encoder;

/// Extension trait for encoding paths to UTF-8 strings.
///
/// This is a separate trait from `Utf8Encoder` due to Rust's coherence rules
/// preventing blanket implementations from coexisting with specific type implementations.
pub trait PathUtf8Encoder {
    /// Converts a path to a UTF-8 string, replacing invalid bytes with fallback characters.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_str::PathUtf8Encoder;
    /// use std::path::Path;
    ///
    /// let path = Path::new("hello.txt");
    /// let encoded = path.to_utf8_string();
    /// assert_eq!(encoded, "hello.txt");
    /// ```
    fn to_utf8_string(&self) -> String;
}

impl PathUtf8Encoder for std::path::Path {
    fn to_utf8_string(&self) -> String {
        self.as_os_str().as_encoded_bytes().to_utf8_string()
    }
}

impl PathUtf8Encoder for std::path::PathBuf {
    fn to_utf8_string(&self) -> String {
        self.as_path().to_utf8_string()
    }
}

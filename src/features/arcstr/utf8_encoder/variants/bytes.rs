use arcstr::ArcStr;

use crate::{ArcStrUtf8Encoder, Utf8Encoder};

impl<T: AsRef<[u8]>> ArcStrUtf8Encoder for T {
    /// Converts a byte sequence into an [`ArcStr`], replacing invalid bytes
    /// with fallback characters.
    ///
    /// Entirely valid UTF-8 input is copied into the [`ArcStr`] directly;
    /// input containing invalid bytes is decoded via
    /// [`Utf8Encoder::to_utf8_string`] first.
    ///
    /// # Example
    ///
    /// ```
    /// use ps_str::ArcStrUtf8Encoder;
    ///
    /// let bytes = b"Hello\xFFWorld";
    /// let result = bytes.to_utf8_arcstr();
    ///
    /// assert_eq!(result, "Hello˙World");
    /// ```
    fn to_utf8_arcstr(&self) -> ArcStr {
        let bytes = self.as_ref();

        std::str::from_utf8(bytes)
            .map_or_else(|_| ArcStr::from(bytes.to_utf8_string()), ArcStr::from)
    }
}

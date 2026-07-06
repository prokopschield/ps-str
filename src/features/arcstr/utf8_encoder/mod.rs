mod variants;

use arcstr::ArcStr;

pub trait ArcStrUtf8Encoder {
    /// Converts a byte sequence into an [`ArcStr`], replacing invalid bytes
    /// with fallback characters from the [`CHARS`](crate::CHARS) table.
    fn to_utf8_arcstr(&self) -> ArcStr;
}

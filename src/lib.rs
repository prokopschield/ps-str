pub mod chars;
pub use chars::CHARS;

#[cfg(test)]
pub mod tests;

pub trait Utf8Encoder {
    fn to_utf8_string(&self) -> String;
}

impl<T: AsRef<[u8]>> Utf8Encoder for T {
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
                        result.push_str(unsafe { std::str::from_utf8_unchecked(valid_part) });
                        i += e.valid_up_to();
                    }

                    result.push(CHARS[bytes[i] as usize]);
                    i += 1;
                }
            }
        }

        result
    }
}

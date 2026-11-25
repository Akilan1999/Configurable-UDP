use bincode::error::{DecodeError, EncodeError};



/// Error values for bincode module.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Detects decoding error from binary vector of bytes.
    Decode(DecodeError),
    /// Detects encoding error from binary vector of bytes.
    Encode(EncodeError),
}

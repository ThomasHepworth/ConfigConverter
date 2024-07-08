mod cli;
mod decode;
mod encode;

pub use decode::{file_path_to_decoder, DecodeError, Decoder, FileReadError};
pub use encode::{EncodeError, Encoder};

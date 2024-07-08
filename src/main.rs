mod cli;
mod decode;
mod encode;

use cli::run_cli;
use decode::{file_path_to_decoder, Decoder};
use encode::Encoder;
use serde_json;
use toml;

pub use decode::DecodeError;
pub use encode::EncodeError;

fn main() {
    // let path_to_decode = file_path_to_decoder("test.txt");
    // match path_to_decode {
    //     Ok(decoder) => {
    //         println!("Decoded");
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //     }
    // }

    match run_cli() {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}

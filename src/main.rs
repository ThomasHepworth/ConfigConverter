mod cli;
mod decode;
mod encode;

use cli::run_cli;

pub use decode::DecodeError;
pub use encode::EncodeError;

fn main() {
    match run_cli() {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
}

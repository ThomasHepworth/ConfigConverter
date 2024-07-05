mod interface;

use crate::decode::Decoder;
use crate::encode::Encoder;
use clap::Parser;
use interface::{Converters, InputOutput, CLI};

pub fn run_cli() {
    let cli_args = CLI::parse();
    let (from, to) = cli_args.converters.conversion_types();
    let io = cli_args.converters.get_input_output();

    let user_message = create_conversion_message(&from.to_string(), &to.to_string(), &io);

    // Print a message declaring the conversion about to take place
    println!("{}", user_message);

    convert_config_file(from, to, io);
}

fn create_conversion_message(from: &str, to: &str, io: &InputOutput) -> String {
    let write_location_msg = if let Some(out) = &io.write {
        format!(" and outputting to '{}'", out)
    } else {
        String::from(".")
    };

    let main_str = format!(
        "🧙 Converting '{}' to {}{}",
        &io.input, to, write_location_msg,
    );
    let line_padding = "-".repeat(main_str.chars().count());

    format!("{}\n{}\n{}", line_padding, main_str, line_padding)
}

fn convert_config_file(from: Decoder, to: Encoder, io: &InputOutput) {
    match from.decode_file(&io.input) {
        Ok(decoded) => {
            if let Some(output_path) = &io.write {
                match to.encode_to_file(&decoded, output_path) {
                    Ok(_) => println!(
                        "🎉 Conversion successful! Output written to '{}'",
                        output_path
                    ),
                    Err(e) => eprintln!("🚨 Conversion failed: {}", e),
                }
            } else {
                match to.encode_to_str(&decoded) {
                    Ok(encoded_str) => println!("{}", encoded_str),
                    Err(e) => eprintln!("🚨 Conversion failed: {}", e),
                }
            }
        }
        Err(e) => eprintln!("🚨 Conversion failed: {}", e),
    }
}

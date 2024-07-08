mod interface;

use crate::decode::{file_path_to_decoder, Decoder};
use crate::encode::Encoder;
use clap::Parser;
use interface::CLI;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum CliError {
    EncoderError(String),
    DecoderError(String),
    ConversionError(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CliError::EncoderError(msg) => write!(f, "Encoder error: {}", msg),
            CliError::DecoderError(msg) => write!(f, "Decoder error: {}", msg),
            CliError::ConversionError(msg) => write!(f, "🚨 Conversion failed: {}", msg),
        }
    }
}

impl Error for CliError {}

pub fn run_cli() -> Result<String, Box<dyn Error>> {
    let cli_args = CLI::parse();
    let (input, write) = cli_args.get_input_output();
    let decoder =
        file_path_to_decoder(&input).map_err(|e| CliError::DecoderError(e.to_string()))?;
    let encoder = cli_args.get_encoder();

    let user_message =
        create_conversion_message(&decoder.to_string(), &encoder.to_string(), write.as_deref());

    let config_success_message = convert_config_file(decoder, encoder, &input, write.as_deref())
        .map_err(|e| CliError::ConversionError(e.to_string()))?;

    Ok(format!("{}\n{}", user_message, config_success_message))
}

fn create_conversion_message(from: &str, to: &str, write: Option<&str>) -> String {
    let write_location_msg = if let Some(out) = write {
        format!(" and outputting to '{}'", out)
    } else {
        String::from(".")
    };

    let main_str = format!("🧙 Converting '{}' to {}{}", from, to, write_location_msg,);
    let line_padding = "-".repeat(main_str.chars().count());

    format!("{}\n{}\n{}", line_padding, main_str, line_padding)
}

fn convert_config_file(
    from: Decoder,
    to: Encoder,
    input: &str,
    write: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let decoded = from.decode_file(input)?;
    if let Some(output_path) = write {
        to.encode_to_file(&decoded, output_path)?;
        Ok(format!(
            "🎉 Conversion successful! Output written to '{}'",
            output_path,
        ))
    } else {
        let encoded_str = to.encode_to_str(&decoded)?;
        Ok(encoded_str)
    }
}

use crate::encode::Encoder;
use clap::{Parser, Subcommand};

/// Transforms configuration files between different formats.
/// Provides a safer way to convert and manipulate configuration files,
/// ensuring data integrity and preventing accidental data loss.
#[derive(Parser)]
#[command(
    name = "config_converter",
    about = "Transforms configuration files between different formats.",
    version = "1.0"
)]
pub struct CLI {
    #[command(subcommand)]
    pub converters: Converters,
}

#[derive(Parser)]
pub struct InputOutput {
    /// Input file path
    #[arg(required = true)]
    pub input: String,
    /// Output file path
    #[arg(short, long)]
    pub write: Option<String>,
}

#[derive(Subcommand)]
pub enum Converters {
    ToTOML(InputOutput),
    ToJSON(InputOutput),
    ToYAML(InputOutput),
}

impl Converters {
    pub fn get_encoder(&self) -> Encoder {
        match self {
            Converters::ToTOML(_) => Encoder::TOML,
            Converters::ToJSON(_) => Encoder::JSON,
            Converters::ToYAML(_) => Encoder::YAML,
        }
    }

    pub fn get_input_output(&self) -> &InputOutput {
        match self {
            Converters::ToTOML(input_output) => input_output,
            Converters::ToJSON(input_output) => input_output,
            Converters::ToYAML(input_output) => input_output,
        }
    }
}

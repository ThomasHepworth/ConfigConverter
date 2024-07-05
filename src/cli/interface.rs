use crate::decode::Decoder;
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
    // TOML CONVERTERS //
    /// Convert from TOML to JSON.
    TomlToJson(InputOutput),
    /// Convert from TOML to YAML.
    TomlToYaml(InputOutput),

    // JSON CONVERTERS //
    /// Convert from JSON to TOML.
    JsonToToml(InputOutput),
    /// Convert from JSON to YAML.
    JsonToYaml(InputOutput),

    // YAML CONVERTERS //
    /// Convert from YAML JSON.
    YamlToJson(InputOutput),
    /// Convert from YAML TOML.
    YamlToToml(InputOutput),
}

impl Converters {
    pub fn conversion_types(&self) -> (Decoder, Encoder) {
        match self {
            Converters::TomlToJson(_) => (Decoder::TOML, Encoder::JSON),
            Converters::TomlToYaml(_) => (Decoder::TOML, Encoder::YAML),
            Converters::JsonToToml(_) => (Decoder::JSON, Encoder::TOML),
            Converters::JsonToYaml(_) => (Decoder::JSON, Encoder::YAML),
            Converters::YamlToJson(_) => (Decoder::YAML, Encoder::JSON),
            Converters::YamlToToml(_) => (Decoder::YAML, Encoder::TOML),
        }
    }

    pub fn get_input_output(&self) -> &InputOutput {
        match self {
            Converters::TomlToJson(input_output) => input_output,
            Converters::TomlToYaml(input_output) => input_output,
            Converters::JsonToToml(input_output) => input_output,
            Converters::JsonToYaml(input_output) => input_output,
            Converters::YamlToJson(input_output) => input_output,
            Converters::YamlToToml(input_output) => input_output,
        }
    }
}

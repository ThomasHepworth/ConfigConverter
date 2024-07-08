use crate::encode::Encoder;
use clap::{Parser, Subcommand};

/// Transforms configuration files between different formats.
/// Provides a safer way to convert and manipulate configuration files,
/// ensuring data integrity and preventing accidental data loss.
#[derive(Parser)]
#[command(
    name = "ConfigConverter",
    about = "Transforms configuration files between different formats.",
    version = "1.0"
)]
pub struct CLI {
    /// Input file path
    #[arg(required = true)]
    pub input: String,

    /// Output file path
    #[arg(short, long)]
    pub write: Option<String>,

    #[command(subcommand)]
    pub converters: Converters,
}

#[derive(Subcommand)]
pub enum Converters {
    ToTOML,
    ToJSON,
    ToYAML,
}

impl CLI {
    pub fn get_encoder(&self) -> Encoder {
        match self.converters {
            Converters::ToTOML => Encoder::TOML,
            Converters::ToJSON => Encoder::JSON,
            Converters::ToYAML => Encoder::YAML,
        }
    }

    pub fn get_input_output(&self) -> (&String, &Option<String>) {
        (&self.input, &self.write)
    }
}

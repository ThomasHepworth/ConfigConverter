use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use thiserror::Error;
use toml::Value as TomlValue;

#[derive(Error, Debug)]
pub enum FileReadError {
    #[error("File read error: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Invalid file path '{invalid}'. Expected a file ending in {expected}")]
    InvalidExtension { invalid: String, expected: String },
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("-------------------------------------\n{0}")]
    Toml(#[from] toml::de::Error),
    #[error("-------------------------------------\n{0}")]
    Json(#[from] serde_json::Error),
    #[error("-------------------------------------\n{0}")]
    Yaml(#[from] serde_yaml::Error),
}

#[derive(strum_macros::Display)]
pub enum Decoder {
    TOML,
    JSON,
    YAML,
}

// Factory function to create a Decoder from a file extension
pub fn file_path_to_decoder(file_path: &str) -> Result<Decoder, FileReadError> {
    let file_extension: Option<&str> = file_path.split('.').last();

    match file_extension {
        Some("toml") => Ok(Decoder::TOML),
        Some("json") => Ok(Decoder::JSON),
        Some("yaml") | Some("yml") => Ok(Decoder::YAML),
        _ => Err(FileReadError::InvalidExtension {
            invalid: file_path.to_string(),
            expected: String::from("'toml', 'json', 'yaml' or 'yml'."),
        }),
    }
}

impl Decoder {
    pub fn decode_from_str(&self, content: &str) -> Result<JsonValue, DecodeError> {
        match self {
            Decoder::TOML => {
                let toml_value: TomlValue = content.parse()?;
                let json_value = serde_json::to_value(toml_value)?;
                Ok(json_value)
            }
            Decoder::JSON => {
                let json_value: JsonValue = serde_json::from_str(content)?;
                Ok(json_value)
            }
            Decoder::YAML => {
                let yaml_value: YamlValue = serde_yaml::from_str(content)?;
                let json_value = serde_json::to_value(yaml_value)?;
                Ok(json_value)
            }
        }
    }

    pub fn decode_file(&self, file_path: &str) -> Result<JsonValue, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        Ok(self.decode_from_str(&content)?)
    }
}

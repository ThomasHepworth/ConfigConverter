use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use thiserror::Error;
use toml::Value as TomlValue;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("-------------------------------------\n{0}")]
    Toml(#[from] toml::de::Error),
    #[error("-------------------------------------\n{0}")]
    Json(#[from] serde_json::Error),
    #[error("-------------------------------------\n{0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("File read error: {0}")]
    FileRead(#[from] std::io::Error),
}

// TODO: INI, YAML
#[derive(strum_macros::Display)]
pub enum Decoder {
    TOML,
    JSON,
    YAML,
}

impl Decoder {
    fn verify_file_extension(&self, file_path: &str) -> bool {
        match self {
            Decoder::TOML => file_path.ends_with(".toml"),
            Decoder::JSON => file_path.ends_with(".json"),
            Decoder::YAML => file_path.ends_with(".yaml") || file_path.ends_with(".yml"),
        }
    }

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

    pub fn decode_file(&self, file_path: &str) -> Result<JsonValue, DecodeError> {
        if !self.verify_file_extension(file_path) {
            return Err(DecodeError::FileRead(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid file extension",
            )));
        }

        let content = fs::read_to_string(file_path)?;
        self.decode_from_str(&content)
    }
}

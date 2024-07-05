use serde_json::Value as JsonValue;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("TOML encode error: {0}")]
    Toml(#[from] toml::ser::Error),
    #[error("JSON encode error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML encode error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("File write error: {0}")]
    FileWrite(#[from] std::io::Error),
}

#[derive(strum_macros::Display)]
pub enum Encoder {
    TOML,
    JSON,
    YAML,
}

impl Encoder {
    pub fn encode_to_str(&self, json_value: &JsonValue) -> Result<String, EncodeError> {
        match self {
            Encoder::TOML => {
                let toml_value: toml::Value = serde_json::from_value(json_value.clone())?;
                let toml_string = toml::to_string(&toml_value)?;
                Ok(toml_string)
            }
            Encoder::JSON => {
                let json_string = serde_json::to_string_pretty(json_value)?;
                Ok(json_string)
            }
            Encoder::YAML => {
                let yaml_value: serde_yaml::Value = serde_json::from_value(json_value.clone())?;
                let yaml_string = serde_yaml::to_string(&yaml_value)?;
                Ok(yaml_string)
            }
        }
    }

    pub fn encode_to_file(
        &self,
        json_value: &JsonValue,
        file_path: &str,
    ) -> Result<(), EncodeError> {
        let content = self.encode_to_str(json_value)?;
        fs::write(file_path, content).map_err(EncodeError::FileWrite)
    }
}

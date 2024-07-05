use config_converter::Encoder;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_encode_to_toml_str() {
    let json_value = serde_json::json!({
        "package": {
            "name": "hello_world",
            "version": "0.1.0"
        }
    });
    let encoded_toml = Encoder::TOML.encode_to_str(&json_value).unwrap();
    assert!(encoded_toml.contains("name = \"hello_world\""));
}

#[test]
fn test_encode_to_json_str() {
    let json_value = serde_json::json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let encoded_json = Encoder::JSON.encode_to_str(&json_value).unwrap();
    assert!(encoded_json.contains("\"name\": \"John Doe\""));
}

#[test]
fn test_encode_to_yaml_str() {
    let json_value = serde_json::json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    let encoded_yaml = Encoder::YAML.encode_to_str(&json_value).unwrap();
    assert!(encoded_yaml.contains("name: John Doe"));
}

#[test]
fn test_encode_to_file() {
    let json_value = serde_json::json!({
        "package": {
            "name": "hello_world",
            "version": "0.1.0"
        }
    });

    // Create a temporary file
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    Encoder::TOML
        .encode_to_file(&json_value, file_path)
        .unwrap();
    let encoded_content = fs::read_to_string(file_path).unwrap();
    assert!(encoded_content.contains("name = \"hello_world\""));
}

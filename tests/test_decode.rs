use config_converter::{DecodeError, Decoder};

#[test]
fn test_invalid_toml() {
    let test_toml: &str = r#"
    [package
    "#;

    let result = Decoder::TOML.decode_from_str(test_toml);

    // Check that an error is returned
    assert!(result.is_err());

    let expected_error = "TOML parse error at line 2, column 13";
    if let Err(DecodeError::Toml(err)) = result {
        let err_str = err.to_string();
        assert!(
            err_str.contains(expected_error),
            "Unexpected error: {}. Expected it to contain: {}",
            err_str,
            expected_error
        );
    } else {
        panic!("Expected a TOML parse error");
    }
}

macro_rules! decode_tests {
    ($($name:ident: ($filename:expr, $decoder:expr),)*) => {
    $(
        #[test]
        fn $name() {
            let file_path = format!("tests/test_data/{}", $filename);
            let decoded = $decoder.decode_file(&file_path).unwrap();

            assert_eq!(
                decoded["tool"]["poetry"]["name"].as_str().unwrap(),
                "my_project"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["version"].as_str().unwrap(),
                "0.1.0"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["description"].as_str().unwrap(),
                "A detailed description of my project."
            );
            assert_eq!(
                decoded["tool"]["poetry"]["authors"][0].as_str().unwrap(),
                "John Doe <john.doe@example.com>"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["license"].as_str().unwrap(),
                "MIT"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["readme"].as_str().unwrap(),
                "README.md"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["homepage"].as_str().unwrap(),
                "https://example.com"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["repository"].as_str().unwrap(),
                "https://github.com/example/my_project"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["documentation"].as_str().unwrap(),
                "https://docs.example.com"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["keywords"][0].as_str().unwrap(),
                "example"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["keywords"][1].as_str().unwrap(),
                "project"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["keywords"][2].as_str().unwrap(),
                "poetry"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dependencies"]["python"]
                    .as_str()
                    .unwrap(),
                "^3.9"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dependencies"]["requests"]
                    .as_str()
                    .unwrap(),
                "^2.25.1"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dependencies"]["numpy"]
                    .as_str()
                    .unwrap(),
                "^1.20.3"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dev-dependencies"]["pytest"]
                    .as_str()
                    .unwrap(),
                "^6.2.4"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dev-dependencies"]["black"]
                    .as_str()
                    .unwrap(),
                "^21.6b0"
            );
            assert_eq!(
                decoded["tool"]["poetry"]["dev-dependencies"]["isort"]
                    .as_str()
                    .unwrap(),
                "^5.9.3"
            );
        }
    )*
    }
}

decode_tests! {
    test_read_toml_file: ("pyproject.toml", Decoder::TOML),
    test_read_yaml_file: ("pyproject.yaml", Decoder::YAML),
    test_read_json_file: ("pyproject.json", Decoder::JSON),
}

#[test]
fn test_decode_fails_for_invalid_file_extension() {
    let decoder = Decoder::TOML;
    let result = decoder.decode_file("invalid_file.txt");
    assert!(result.is_err());
}
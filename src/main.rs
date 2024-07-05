mod cli;
mod decode;
mod encode;

use cli::run_cli;
use decode::Decoder;
use encode::Encoder;
use serde_json;
use toml;

pub use decode::DecodeError;
pub use encode::EncodeError;

// fn toml_to_json(toml_str: &str) -> Result<String, Box<dyn std::error::Error>> {
//     // Parse TOML string into TomlValue
//     let toml_value: toml::Value = toml_str.parse()?;
//     println!("TOML:\n{:?}", toml_value);

//     // Convert TomlValue to JSON string
//     let json_string = serde_json::to_string_pretty(&toml_value)?;
//     Ok(json_string)
// }

// fn main() {
//     let toml_str = r#"
//         [package]
//         name = "example"
//         version = "0.1.0"
//     "#;

//     match toml_to_json(toml_str) {
//         Ok(json) => println!("{}", json),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }

fn main() {
    // let test_toml: &str = r#"
    // [package
    // "#;

    // let result = Decoder::TOML.decode_from_str(test_toml);
    // println!("{:#?}", result);
    // match result {
    //     Ok(_) => println!("No error"),
    //     Err(e) => eprintln!("{}", e),
    // }

    // println!("{}", Decoder::TOML.to_string());
    run_cli();

    // let toml_str = r#"
    //     [package]
    //     name = "example"
    //     version = "0.1.0"
    // "#;

    // // let json_value = decode::decode_from_str(&Decoder::TOML, toml_str).unwrap();
    // let json_value = Decoder::TOML.decode_from_str(toml_str).unwrap();
    // println!("DECODED VALUE:\n{}", json_value);

    // let json_out = Encoder::JSON.encode_to_str(&json_value).unwrap();
    // println!("ENCODED VALUE:\n{}", json_out);
}

use serde_json;
use std::fs;

pub fn get_data_as_array(file_path: String) -> Vec<String> {
    let data = fs::read_to_string(file_path).expect("file to read successfully");
    let json: serde_json::Value = serde_json::from_str(&data).expect("json to parse successfully");
    json.as_array()
        .unwrap()
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

use crate::helpers::get_data_as_array;
use std::process::exit;

pub fn run() -> String {
    let data = get_data_as_array("../day2example.json".to_string());
    println!("{:?}", data);
    exit(0);
    return String::from("Not started!");
}

use serde_json::Value;
use std::fs;

pub fn read_json_file(filename: String) -> Vec<String> {
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let v: Value = serde_json::from_str(&contents).unwrap();

    // Access parts of the data by indexing with square brackets.
    let mut cities: Vec<String> = Vec::new();

    for city in v.as_array().unwrap().iter() {
        cities.push(city["name"].as_str().unwrap().to_lowercase());
    }

    cities
}

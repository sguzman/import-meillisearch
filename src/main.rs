use clap::{Arg, Command};
use serde_json::{Value, json};
use std::fs;
use std::process;

fn main() {
    // Define the CLI arguments using clap
    let matches = Command::new("JSON ID Inserter")
        .version("1.0")
        .author("Your Name")
        .about("Adds unique IDs to JSON objects in an array")
        .arg(
            Arg::new("input")
                .about("The JSON array string or path to a JSON file")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .get_matches();

    // Get the input argument
    let input = matches.value_of("input").unwrap();

    // Read the JSON input (either from a file or directly as a string)
    let input_json = if let Ok(file_content) = fs::read_to_string(input) {
        file_content
    } else {
        input.to_string()
    };

    // Parse the JSON array
    let mut json_array: Vec<Value> = match serde_json::from_str(&input_json) {
        Ok(array) => array,
        Err(_) => {
            eprintln!("Invalid JSON array provided.");
            process::exit(1);
        }
    };

    // Add a unique `id` field to each object in the array
    for (i, obj) in json_array.iter_mut().enumerate() {
        if let Some(map) = obj.as_object_mut() {
            map.insert("id".to_string(), json!(i + 1));
        }
    }

    // Print the modified JSON array
    println!("{}", serde_json::to_string_pretty(&json_array).unwrap());
}

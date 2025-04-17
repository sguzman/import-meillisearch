use serde_json::{Value, json};
use std::env;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a JSON string is provided as an argument
    if args.len() < 2 {
        eprintln!("Usage: {} '<json_array>'", args[0]);
        std::process::exit(1);
    }

    // Parse the JSON array from the command-line argument
    let input_json = &args[1];
    let mut json_array: Vec<Value> = match serde_json::from_str(input_json) {
        Ok(array) => array,
        Err(_) => {
            eprintln!("Invalid JSON array provided.");
            std::process::exit(1);
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

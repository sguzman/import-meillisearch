# JSON ID Adder

**JSON ID Adder** is a command-line tool written in Rust that adds a unique ID field to each object in a JSON array. It supports input from a JSON file and allows customization of the key name for the ID field.

## Features

- Adds a unique ID to each object in a JSON array.
- Supports input from a JSON file.
- Allows customization of the key name for the ID field (default is `id`).

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/import-meil.git
   cd import-meil
   ```
2. Build the project using Cargo:f
    ```bash
    cargo build --release
    ```
3. The compiled binary will be available in the `release` directory:
    ```bash
    ./target/release/import-meil
    ``

## Usage

Command-line Arguments

* `--input <FILE>`: Path to the input JSON file (required).

* `--name <KEY>`: Optional key name for the ID field (default: id).

### Example Commands

#### Add IDs to a JSON file

```bash
./import-meil --input data.json
```

#### Customize the key name for the ID field

```bash
./import-meil --input data.json --name custom_id
```

#### Input Example (`data.json`)

>[
    { "name": "Alice" },
    { "name": "Bob" }
]

#### Output Example

>[
    {
        "name": "Alice",
        "id": 1
    },
    {
        "name": "Bob",
        "id": 2
    }
]

## Dependencies

* `clap`: For parsing command-line arguments.

* `serde` and `serde_json`: For JSON parsing and manipulation.

## License

This project is licensed under the CC0 1.0 License. See the LICENSE file for details.

## Author

Created by Salvador Guzman.
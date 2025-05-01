📦 JSON to CSV Converter (Rust)
This is a simple command-line utility written in Rust that converts a JSON array of objects into a CSV file. It uses:

clap for CLI argument parsing

serde_json for parsing JSON

csv for writing CSV

🛠️ Installation
Make sure you have Rust installed. Then clone this repo and build the project:
```
git clone https://github.com/surenidh/file-convertor.git
cd json-to-csv-rs
cargo build --release
```
🚀 Usage
```
cargo run -- --input data.json --output data.csv
```

📥 Example Input (data.json)
```
[
  {"name": "Alice", "age": 30},
  {"name": "Bob", "age": 25, "city": "NYC"}
]

```

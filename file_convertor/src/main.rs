use clap::Parser;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Args {
    /// Input JSON file path
    #[arg(short, long)]
    input: String,

    /// Output CSV file path
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read JSON
    let file = File::open(&args.input)?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;

    let arr = data.as_array().ok_or("Expected a JSON array")?;

    let mut wtr = csv::Writer::from_path(&args.output)?;

    // Collect all keys as headers
    let headers: Vec<String> = arr.iter()
        .flat_map(|obj| obj.as_object().unwrap().keys().cloned())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    wtr.write_record(&headers)?;

    for obj in arr {
        let row = headers.iter().map(|k| {
            obj.get(k).map_or("".to_string(), |v| v.to_string())
        });
        wtr.write_record(row)?;
    }

    wtr.flush()?;
    println!("✅ Converted {} -> {}", args.input, args.output);

    Ok(())
}

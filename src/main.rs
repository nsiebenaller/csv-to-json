#![allow(dead_code)]
mod config;
pub mod json;
pub mod schema;

use csv::StringRecord;
use serde_json::{Map, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const HAS_HEADERS: bool = true;

fn main() {
    // Benchmark
    let start = std::time::Instant::now();

    // Get command line args
    let config = config::init();

    // Find input file
    let mut reader = match csv::Reader::from_path(config.input) {
        Ok(reader) => reader,
        Err(_) => {
            println!("Invalid input path");
            exit(1);
        }
    };

    // Read header row
    let headers = match reader.headers() {
        Ok(headers) => headers,
        Err(_) => {
            println!("Failed reading headers");
            exit(1);
        }
    };

    // Create json object 'keys'
    let mut keys = Vec::new();
    for header in headers {
        keys.push(header.to_string());
    }

    // Read all lines into json objects
    let mut json_array = Vec::new();
    for (row, line_result) in reader.records().into_iter().enumerate() {
        match line_result {
            Ok(line) => match record_to_json(&keys, &line, &config.schema) {
                Ok(entry) => json_array.push(entry),
                Err(err_msg) => {
                    println!("Error processing CSV row {:?} ERR: {:?}", row, err_msg);
                }
            },
            Err(_) => {
                println!("Failed reading row");
            }
        }
    }

    // Write json to file
    let output = match serde_json::to_string(&json_array) {
        Ok(output) => output,
        Err(_) => {
            println!("Failed converting json to string");
            exit(1);
        }
    };
    let mut file = match File::create(config.output.clone()) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed creating new output file");
            exit(1);
        }
    };
    match file.write_all(output.as_bytes()) {
        Ok(_) => println!("Created JSON file."),
        Err(_) => println!(
            "Failed creating new JSON file '{:?}'",
            config.output.clone()
        ),
    }

    println!("Done! ({:?})", start.elapsed());
    println!("");
}

fn record_to_json(
    keys: &Vec<String>,
    record: &StringRecord,
    schema: &Option<schema::Schema>,
) -> Result<Value, String> {
    let mut json = Map::new();
    for (i, value) in record.iter().enumerate() {
        let key = match keys.get(i) {
            Some(key) => key,
            None => {
                return Err(format!("Extra column found {:?}", i));
            }
        };
        json.insert(alias(key, schema), Value::String(value.to_string()));
    }
    return Ok(Value::Object(json.to_owned()));
}

fn alias(key: &String, schema: &Option<schema::Schema>) -> String {
    match schema {
        Some(fields) => {
            for field in fields {
                if key.eq(&field.alias) {
                    return field.name.to_string();
                }
            }
            String::from(key.to_owned())
        }
        None => String::from(key.to_owned()),
    }
}

fn get_env(names: &Vec<&str>) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    for (index, arg) in args.iter().enumerate() {
        for name in names {
            if arg.eq(name) {
                return match args.get(index + 1) {
                    Some(value) => Some(value.to_string()),
                    None => None,
                };
            }
        }
    }
    return None;
}

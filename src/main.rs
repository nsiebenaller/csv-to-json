#![allow(dead_code)]
mod config;
pub mod json;
pub mod schema;

use csv::StringRecord;
use schema::{Schema, SchemaFieldType};
use serde_json::{Map, Value};
use std::collections::HashMap;
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
    headers: &Vec<String>,
    record: &StringRecord,
    schema: &Option<Schema>,
) -> Result<Value, String> {
    let mut json = Map::new();

    // Seed json with Schema
    match schema {
        Some(schema_fields) => {
            for schema_field in schema_fields {
                match schema_field.field_type {
                    SchemaFieldType::String => {
                        // Check if header should be loaded as value
                        if schema_field.header {
                            for header in headers {
                                match schema_field.match_alias(header.to_string()) {
                                    true => {
                                        json.insert(
                                            schema_field.name.to_string(),
                                            Value::String(header.to_string()),
                                        );
                                    }
                                    false => {
                                        json.insert(schema_field.name.to_string(), Value::Null);
                                    }
                                }
                            }
                        } else {
                            json.insert(schema_field.name.to_string(), Value::Null);
                        }
                    }
                    SchemaFieldType::Array => {
                        json.insert(schema_field.name.to_string(), Value::Array(Vec::new()));
                    }
                    _ => {
                        json.insert(schema_field.name.to_string(), Value::Null);
                    }
                };
            }
        }
        None => {}
    }

    for (index, value) in record.iter().enumerate() {
        let header = match headers.get(index) {
            Some(header) => header,
            None => {
                return Err(format!("Extra column found {:?}", index));
            }
        };

        // Check if Schema is defined
        match schema {
            Some(schema_fields) => {
                // @TODO: Extract this section for recursion (deep nested schema objects)
                // Check Type of schema fields
                for schema_field in schema_fields {
                    match schema_field.field_type {
                        SchemaFieldType::String => {
                            if !schema_field.header {
                                match schema_field.match_alias(header.to_string()) {
                                    true => {
                                        json.insert(
                                            schema_field.name.to_string(),
                                            Value::String(value.to_string()),
                                        );
                                    }
                                    false => {}
                                }
                            }
                        }
                        SchemaFieldType::Array => {
                            // @TODO: Handle array of strings
                            // @TODO: Handle array of numbers

                            // Impl: Array of objects
                            let mut header_map: HashMap<String, Map<String, Value>> =
                                HashMap::new();
                            for property in &schema_field.properties {
                                // Check if any properties match header
                                if property.match_alias(header.to_string()) {
                                    // Insert property into array
                                    match header_map.get_mut(header) {
                                        Some(json_object) => {
                                            let json_value = if property.header {
                                                header.to_string()
                                            } else {
                                                value.to_string()
                                            };
                                            json_object.insert(
                                                property.name.to_string(),
                                                Value::String(json_value),
                                            );
                                        }
                                        None => {
                                            // First instance of property
                                            let mut new_map = Map::new();
                                            let json_value = if property.header {
                                                header.to_string()
                                            } else {
                                                value.to_string()
                                            };
                                            new_map.insert(
                                                property.name.to_string(),
                                                Value::String(json_value),
                                            );
                                            header_map.insert(header.to_string(), new_map);
                                        }
                                    };
                                }
                            }

                            let mut json_array = Vec::new();
                            for array_element in header_map.values() {
                                json_array.push(Value::Object(array_element.clone()));
                            }
                            json.insert(schema_field.name.to_string(), Value::Array(json_array));
                        }
                        _ => { /* Unsupported SchemaFieldType */ }
                    }
                }
            }
            None => {
                json.insert(header.to_string(), Value::String(value.to_string()));
            }
        }
    }
    return Ok(Value::Object(json.to_owned()));
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

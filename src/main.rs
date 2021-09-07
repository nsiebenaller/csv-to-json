#![allow(dead_code)]
use csv::StringRecord;
use serde_json::{Map, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::exit;

const HAS_HEADERS: bool = true;

fn main() {
    // Get command line args
    let input_path = match get_env(&vec!["-I", "-input"]) {
        Some(input_path) => input_path,
        None => {
            println!("No input path given");
            exit(1);
        }
    };
    let output_path = match get_env(&vec!["-O", "-output"]) {
        Some(output_path) => output_path,
        None => {
            println!("No output path given");
            exit(1);
        }
    };

    // Find input file
    let mut reader = match csv::Reader::from_path(input_path) {
        Ok(reader) => reader,
        Err(_) => {
            println!("Invalid path");
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
            Ok(line) => match record_to_json(&keys, &line) {
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
    let mut file = match File::create(output_path.clone()) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed creating new output file");
            exit(1);
        }
    };
    match file.write_all(output.as_bytes()) {
        Ok(_) => println!("Created new JSON file '{:?}'", output_path.clone()),
        Err(_) => println!("Failed creating new JSON file '{:?}'", output_path.clone()),
    }

    println!("Done!");
}

fn record_to_json(keys: &Vec<String>, record: &StringRecord) -> Result<Value, String> {
    let mut json = Map::new();
    for (i, value) in record.iter().enumerate() {
        let key = match keys.get(i) {
            Some(key) => key,
            None => {
                return Err(format!("Extra column found {:?}", i));
            }
        };
        json.insert(key.to_string(), Value::String(value.to_string()));
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

pub struct AverageCollection {
    list: Vec<usize>,
    average: f64,
}
impl AverageCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0.0,
        }
    }
    pub fn push(&mut self, value: usize) {
        self.list.push(value);
        self.update_average();
    }
    pub fn pop(&mut self) {
        self.list.pop();
        self.update_average();
    }
    fn update_average(&mut self) {
        let length = self.list.len() as f64;
        let mut sum = 0.0;
        for num in &self.list {
            sum += *num as f64;
        }
        self.average = sum / length;
    }
}

#[cfg(test)]
mod tests {
    use crate::AverageCollection;

    #[test]
    fn new() {
        let avg_coll = AverageCollection::new();
        assert_eq!(avg_coll.average, 0.0);
    }

    #[test]
    fn push() {
        let mut avg_coll = AverageCollection::new();
        avg_coll.push(3);
        assert_eq!(avg_coll.average, 3.0);
    }
}

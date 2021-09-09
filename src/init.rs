use serde_json;
use std::env;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};
use std::string::String;

const CONFIG_PATH: &str = "./config.json";

pub fn init() -> (String, String) {
    // Show help
    match env_exists(&vec!["-H", "-help"]) {
        true => show_help(),
        false => {}
    };

    let mut input_path = String::from(""); // Required
    let mut output_path = String::from(""); // Required

    // Check for config file
    let config_path = match get_env(&vec!["-C", "-config"]) {
        Some(config_path) => config_path,
        None => String::from(CONFIG_PATH),
    };
    match read_json_file(config_path) {
        Err(_) => println!("No config loaded."),
        Ok(config) => {
            let input = config.get("input");
            if input.is_some() {
                input_path = input.unwrap().as_str().unwrap().to_string();
            }
            let output = config.get("output");
            if output.is_some() {
                output_path = output.unwrap().as_str().unwrap().to_string();
            }
        }
    }

    // Get command line args
    match get_env(&vec!["-I", "-input"]) {
        Some(input) => input_path = input,
        _ => {}
    };
    match get_env(&vec!["-O", "-output"]) {
        Some(output) => output_path = output,
        _ => {}
    };

    if input_path.is_empty() {
        panic!("No input path given. For help run `csv-to-json -help`");
    }
    if output_path.is_empty() {
        panic!("No output path given. For help run `csv-to-json -help`");
    }

    // Read out env variables
    println!("");
    println!("Input Path: {:?}", input_path);
    println!("Output Path: {:?}", output_path);

    return (input_path, output_path);
}

fn env_exists(names: &Vec<&str>) -> bool {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        for name in names {
            if arg.eq(name) {
                return true;
            }
        }
    }
    return false;
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

fn read_json_file(path: String) -> Result<serde_json::Map<String, serde_json::Value>, Error> {
    let file = File::open(path).expect("Cannot find json file");
    let reader = BufReader::new(file);

    return match serde_json::from_reader(reader) {
        Ok(json) => Ok(json),
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse json file",
            ))
        }
    };
}

fn show_help() {
    println!("");
    println!("csv-to-json");
    println!("");
    println!("USAGE");
    println!("\x20\x20$ csv-to-json [ARGUMENTS]");
    println!("");
    println!("ARGUMENTS");
    println!("\x20\x20-I, -input      Input path of CSV");
    println!("\x20\x20-O, -output     Outputh path of JSON");
    println!("\x20\x20-C, -config     Path to JSON config file");
    println!("\x20\x20-H, -help       Display help for csv-to-json");
    println!("");
    std::process::exit(1);
}

use std::env;
use std::string::String;

use crate::json::{self};
use crate::schema::{self, Schema};

const CONFIG_PATH: &str = "./config.json";

pub struct Config {
    pub input: String,
    pub output: String,
    pub schema: Option<Schema>,
}
pub fn init() -> Config {
    // Show help
    match env_exists(&vec!["-H", "-help"]) {
        true => show_help(),
        false => {}
    };

    let mut config = Config {
        input: String::from(""),
        output: String::from(""),
        schema: None,
    };

    // Check for config file
    let config_path = match get_env(&vec!["-C", "-config"]) {
        Some(config_path) => config_path,
        None => String::from(CONFIG_PATH),
    };
    match json::read_json_file(config_path) {
        Err(_) => {}
        Ok(json) => {
            // Load 'input_path'
            json.get_string("input");
            match json.get_string("input") {
                Some(input) => config.input = input,
                None => {}
            }
            // Load 'output_path'
            match json.get_string("output") {
                Some(output) => config.output = output,
                None => {}
            }
            // Load 'schema'
            match json.get_object("schema") {
                Some(schema_json) => match schema::parse_schema(schema_json) {
                    Ok(schema) => config.schema = Some(schema),
                    Err(_) => {}
                },
                None => {}
            }
        }
    }

    // Get command line args
    match get_env(&vec!["-I", "-input"]) {
        Some(input) => config.input = input,
        _ => {}
    };
    match get_env(&vec!["-O", "-output"]) {
        Some(output) => config.output = output,
        _ => {}
    };

    // Check for required variables
    let missing_input = config.input.is_empty();
    let missing_output = config.output.is_empty();
    if missing_input {
        println!("Missing required parameter 'input'. For help run `csv-to-json -help`.");
    }
    if missing_input {
        println!("Missing required parameter 'input'. For help run `csv-to-json -help`.");
    }
    if missing_input || missing_output {
        std::process::exit(1);
    }

    // Read out env variables
    println!("");
    println!("Input Path: {:?}", config.input);
    println!("Output Path: {:?}", config.output);
    println!(
        "Schema: {:?}",
        if config.schema.is_some() {
            "Loaded!"
        } else {
            "Not Loaded."
        }
    );
    println!("");

    return config;
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

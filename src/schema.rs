use regex::Regex;

use crate::json::{self, JsonObject};
use std::io::Error;

pub type Schema = Vec<SchemaField>;

#[derive(Debug, Clone)]
pub struct SchemaField {
    pub name: String,
    pub alias: Alias,
    pub header: bool,
    pub field_type: SchemaFieldType,
    pub properties: Vec<SchemaField>,
}
impl SchemaField {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            alias: Alias::new(),
            header: false,
            field_type: SchemaFieldType::String,
            properties: Vec::new(),
        }
    }
    pub fn add_alias(&mut self, alias: String) {
        self.alias.aliases.push(alias);
    }
    pub fn add_alias_regex(&mut self, regex: String) {
        // @TODO: Handle invalid regex
        self.alias.regex = Some(Regex::new(&regex).unwrap());
    }
    pub fn match_alias(&self, other: String) -> bool {
        //println!("Match {:?} {:?}", other, self.alias);
        // No aliases setup - fallback to field name
        if self.alias.aliases.is_empty() && self.alias.regex.is_none() {
            return self.name.eq(&other);
        }

        // Match against aliases
        for alias in &self.alias.aliases {
            if alias.eq(&other) {
                return true;
            }
        }

        // Match against regex
        match &self.alias.regex {
            Some(regex) => return regex.is_match(&other),
            None => {}
        }

        return false;
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    aliases: Vec<String>,
    regex: Option<Regex>,
}
impl Alias {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
            regex: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SchemaFieldType {
    String,
    Int,
    Float,
    Array,
}
impl SchemaFieldType {
    pub fn from_string(string: &str) -> Option<SchemaFieldType> {
        match string {
            "string" => Some(SchemaFieldType::String),
            "int" => Some(SchemaFieldType::Int),
            "float" => Some(SchemaFieldType::Float),
            "array" => Some(SchemaFieldType::Array),
            _ => None,
        }
    }
}

pub fn parse_schema(json: JsonObject) -> Result<Schema, Error> {
    let mut schema = Schema::new();

    for (json_key, json_value) in json.get_entries() {
        match parse_schema_field(json_value) {
            Some(mut field) => {
                field.name = json_key.to_string();
                schema.push(field);
            }
            None => {}
        }
    }

    println!("{:?}", schema);

    return Ok(schema);
}

pub fn parse_schema_field(json: &serde_json::Value) -> Option<SchemaField> {
    // Json Value should be an object
    let object = match json::parse_object(json) {
        Some(field) => field,
        None => return None,
    };

    let mut schema_field = SchemaField::new();

    // Check for 'alias'
    match object.get_string("alias") {
        Some(alias) => schema_field.add_alias(alias),
        None => {}
    }

    // Check for 'header'
    schema_field.header = object.get_bool("header").unwrap_or(false);

    // Check for 'regex'
    match object.get_string("regex") {
        Some(regex_str) => schema_field.add_alias_regex(regex_str),
        None => {}
    }

    // Check for 'type' (default: String)
    match object.get_string("type") {
        Some(field_type) => match SchemaFieldType::from_string(&field_type) {
            Some(schema_field_type) => schema_field.field_type = schema_field_type,
            None => {}
        },
        None => {}
    }

    // Check for 'properties' (only used for 'array' & 'object' types)
    let mut properties = Vec::new();
    match object.get_object("properties") {
        Some(object_properties) => {
            for (property_name, property_field) in object_properties.get_entries() {
                // Each property should be a valid 'SchemaField'
                match parse_schema_field(property_field) {
                    Some(mut inner_schema_field) => {
                        inner_schema_field.name = property_name.to_string();
                        properties.push(inner_schema_field);
                    }
                    None => {}
                }
            }
            schema_field.properties = properties;
        }
        None => {}
    }

    return Some(schema_field);
}

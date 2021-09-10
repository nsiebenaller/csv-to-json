use crate::json::{self, JsonObject};
use std::io::Error;

pub type Schema = Vec<SchemaField>;

#[derive(Debug)]
pub struct SchemaField {
    pub name: String,
    pub alias: String,
    pub field_type: SchemaFieldType,
}

#[derive(Debug)]
pub enum SchemaFieldType {
    String,
    Int,
    Float,
}

pub fn parse_schema(json: JsonObject) -> Result<Schema, Error> {
    let mut schema = Schema::new();

    for (field_name, field) in json.inner() {
        let field = match json::parse_object(field) {
            Some(field) => field,
            None => {
                println!("Invalid Schema: fields should be objects");
                std::process::exit(1);
            }
        };

        let mut schema_field = SchemaField {
            name: field_name.to_string(),
            alias: String::from(""),
            field_type: SchemaFieldType::String,
        };

        match field.get_string("alias") {
            Some(alias) => schema_field.alias = alias,
            None => {}
        }

        schema.push(schema_field);
    }

    return Ok(schema);
}

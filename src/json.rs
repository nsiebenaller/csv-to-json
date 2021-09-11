use serde_json::{Map, Value};
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

#[derive(Debug)]
pub struct JsonObject {
    inner: Map<String, Value>,
}

impl JsonObject {
    pub fn inner(&self) -> &Map<String, Value> {
        &self.inner
    }
    pub fn get_string(&self, key: &str) -> Option<String> {
        match self.inner.get(key) {
            Some(value) => match value {
                Value::String(string) => return Some(string.to_string()),
                _ => None,
            },
            None => None,
        }
    }
    pub fn get_object(&self, key: &str) -> Option<JsonObject> {
        match self.inner.get(key) {
            Some(value) => match value {
                Value::Object(inner) => {
                    return Some(JsonObject {
                        inner: inner.to_owned(),
                    })
                }
                _ => None,
            },
            None => None,
        }
    }
    pub fn get_entries(&self) -> serde_json::map::Iter {
        self.inner.iter()
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.inner.get(key) {
            Some(value) => match value {
                Value::Bool(bool) => return Some(*bool),
                _ => None,
            },
            None => None,
        }
    }
}

pub fn parse_string(value: &Value) -> Option<String> {
    match value {
        Value::String(string) => Some(string.to_owned()),
        _ => None,
    }
}
pub fn parse_object(value: &Value) -> Option<JsonObject> {
    match value {
        Value::Object(inner) => {
            return Some(JsonObject {
                inner: inner.to_owned(),
            })
        }
        _ => None,
    }
}

pub fn read_json_file(path: String) -> Result<JsonObject, Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "Cannot find json file")),
    };
    let reader = BufReader::new(file);
    let inner = match serde_json::from_reader(reader) {
        Ok(json) => match json {
            Value::Object(map) => map,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Json should be an object",
                ))
            }
        },
        Err(_) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to parse json file",
            ))
        }
    };
    return Ok(JsonObject { inner });
}

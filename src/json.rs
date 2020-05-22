use std::fs::File;
use std::collections::HashMap;
use serde_json::{Result, Value};

pub fn parse_file(file: &File) -> Result<HashMap<Vec<String>, String>> {
    let v: Value = serde_json::from_reader(file)?;
    let mut contents = HashMap::new();

    parse_value(&v, &mut contents, vec![]);
    Ok(contents)
}

fn parse_value(v: &Value, contents: &mut HashMap<Vec<String>, String>, path: Vec<String>) {
    match v {
        Value::String(val) => {
            // contents.insert(path.to_string(), val.to_string());
            contents.insert(path, val.to_string());
        }
        Value::Object(map) => {
            for key in map.keys() {
                let mut path = path.clone();
                path.push(key.to_string());
                parse_value(map.get(key).unwrap(), contents, path);
            }
        },
        _ => (),
    };
}

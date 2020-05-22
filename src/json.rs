use std::fs::File;
use std::collections::HashMap;
use serde_json::{Result, Value};

pub fn parse_file(file: &File) -> Result<HashMap<String, String>> {
    let v: Value = serde_json::from_reader(file)?;
    let mut contents = HashMap::new();

    parse_value(&v, &mut contents, "");
    Ok(contents)
}

fn parse_value(v: &Value, contents: &mut HashMap<String, String>, path: &str) {
    match v {
        Value::String(val) => {
            contents.insert(path.to_string(), val.to_string());
        }
        Value::Object(map) => {
            for key in map.keys() {
                let path = if path.len() == 0 {
                    key.to_string()
                } else {
                    format!("{}.{}", path, key)
                };
                parse_value(map.get(key).unwrap(), contents, &path);
            }
        },
        _ => (),
    };
}

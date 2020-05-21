use std::env;
use std::fs::File;
extern crate serde_json;
use serde_json::{Result, Value};
use std::collections::HashMap;
use xlsxwriter::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = parse_config(&args);

    let file = File::open(filename)
        .expect(&format!("Can't open file {}", filename));

    let contents = parse_file(&file).unwrap();

    write_to_excel(&contents);
}

fn parse_config(args: &[String]) -> &str {
    &args[1]
}

fn parse_file(file: &File) -> Result<HashMap<String, String>> {
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

fn write_to_excel (contents: &HashMap<String, String>) {
    let wb = Workbook::new("./test.xlsx");
    let mut sheet = wb.add_worksheet(Some("Dictionary")).unwrap();

    let mut keys: Vec<_> = contents.keys().collect();
    keys.sort();

    let mut row = 1;
    for key in keys.iter() {
        let val = contents.get(*key).unwrap();
        
        sheet.write_string(row, 0, key, None)
            .expect("Error write in excel file");
        sheet.write_string(row, 1, val, None)
            .expect("Error write in excel file");

        row += 1;
    }
    
    wb.close().expect("close excel error!");
}

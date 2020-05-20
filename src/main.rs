use std::env;
use std::fs::File;
extern crate serde_json;
use serde_json::{Result, Value};
use std::collections::HashMap;
extern crate simple_excel_writer as excel;

use excel::*;

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
    let mut wb = Workbook::create("./test.xlsx");
    let mut sheet = wb.create_sheet("Dictionary");

    let mut keys: Vec<_> = contents.keys().collect();
    keys.sort();

    wb.write_sheet(&mut sheet, |sw| {
        for key in keys.iter() {
            let val = contents.get(*key).unwrap();
            if let Err(err) = sw.append_row(row![key.as_str(), val.as_str()]) {
                return Err(err);
            }
        }

        Ok(())
    }).expect("write excel error");

    wb.close().expect("close excel error!");
}

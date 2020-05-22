use std::env;
use std::fs::File;
use std::collections::HashMap;
extern crate serde_json;
use serde_json::{Result, Value};
extern crate xlsxwriter;
use xlsxwriter::*;
extern crate clap;
use clap::{App, Arg};

#[derive(Debug)]
struct Config<'a> {
    input: &'a str,
    output: &'a str,
    separator: &'a str,
}

fn main() {
    let matches = App::new("json-to-xlsx")
        .version("0.1.0")
        .author("HummerHead87 <snooks87@gmail.com>")
        .about("convert json files to xlsx tables")
        .arg(
            Arg::with_name("input")
            .short("i")
            .long("input")
            .takes_value(true)
            .value_name("FILE")
            .help("provide a *.json file to parse from")
            .required(true)
        )
        .arg(
            Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .value_name("FILE")
            .default_value("output.xlsx")
            .help("provide a *.xlsx file to write result")
        )
        .arg(
            Arg::with_name("separator")
            .short("sep")
            .long("separator")
            .takes_value(true)
            .default_value(".")
            .help("separator for json field names in output file")
        )
        .get_matches();

    let config = parse_config(&matches);
    println!("{:?}", config);
    
    let file = File::open(config.input)
        .expect(&format!("Can't open file {}", config.input));

    let contents = parse_file(&file).unwrap();

    write_to_excel(&contents, config.output);
}

fn parse_config<'a>(matches: &'a clap::ArgMatches) -> Config<'a> {
    Config {
        output: matches.value_of("output").unwrap(),
        input: matches.value_of("input").unwrap(),
        separator: matches.value_of("separator").unwrap(),
    }
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

fn write_to_excel (contents: &HashMap<String, String>, output: &str) {
    let wb = Workbook::new(output);
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

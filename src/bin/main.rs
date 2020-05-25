use std::fs::File;
use clap::{App, Arg};
use json_xlsx::{helpers, json, xlsx};

fn main() {
    let matches = get_matches();
    let config = helpers::parse_config(matches);
    println!("{:?}", config);
    
    let file = File::open(&config.input).unwrap();

    let contents = json::parse_file(&file).unwrap();
    // println!("{:?}", contents);
    xlsx::write_to_xlsx(&contents, &config);
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("json-to-xlsx")
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
        .arg(
            Arg::with_name("language")
            .short("l")
            .long("language")
            .takes_value(true)
            .default_value("ru")
            .help("language of words in .json input file")
        )
        .get_matches()
}

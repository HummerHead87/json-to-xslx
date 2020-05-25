#[derive(Debug)]
pub struct Config {
    pub input: String,
    pub output: String,
    pub separator: String,
}

pub fn parse_config(matches: clap::ArgMatches) -> Config {
    Config {
        output: matches.value_of("output").unwrap().to_string(),
        input: matches.value_of("input").unwrap().to_string(),
        separator: matches.value_of("separator").unwrap().to_string(),
    }
}

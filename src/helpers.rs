#[derive(Debug)]
pub struct Config<'a> {
    pub input: &'a str,
    pub output: &'a str,
    pub separator: &'a str,
}

pub fn parse_config<'a>(matches: &'a clap::ArgMatches) -> Config<'a> {
    Config {
        output: matches.value_of("output").unwrap(),
        input: matches.value_of("input").unwrap(),
        separator: matches.value_of("separator").unwrap(),
    }
}

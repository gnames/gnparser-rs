#[macro_use]
extern crate clap;

use gnparser::{Format, GNParser};

fn main() {
    use clap::App;
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("gnparser.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(ref input) = matches.value_of("INPUT") {
        let mut gnp = GNParser::new();
        if let Some(format) = matches.value_of("format").map(|f| Format::from(f)) {
            gnp.format = format;
        }
        println!("{}", gnp.parse_and_format(input));
    } else {
        println!("NO INPUT");
    }
}

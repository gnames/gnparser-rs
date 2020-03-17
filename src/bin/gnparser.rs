use std::fs::File;
use std::io::{self, BufRead};
use std::path;
use std::process;

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

        if path::Path::new(input).exists() {
            match parse_file(gnp, input) {
                Ok(_) => process::exit(0),
                Err(err) => {
                    println!("{:#?}", err);
                    process::exit(1);
                }
            }
        } else {
            println!("{}", gnp.parse_and_format(input));
        }
    } else {
        println!("NO INPUT");
    }
}

fn parse_file(mut gnp: GNParser, path: &str) -> io::Result<()> {
    let f = File::open(path)?;
    let lines = io::BufReader::new(f).lines();
    for l in lines {
        if let Ok(res) = l {
            println!("{}", gnp.parse_and_format(&res));
        }
    }
    Ok(())
}

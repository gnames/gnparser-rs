#![warn(clippy::all)]
pub mod output;
pub mod peg;

#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

pub use output::Format;
pub use peg::SciName;
use serde_json;

#[derive(Debug, Default)]
pub struct GNParser {
    pub format: Format,
    pub is_test: bool,
}

impl GNParser {
    pub fn new() -> GNParser {
        GNParser {
            ..Default::default()
        }
    }

    pub fn parse(&mut self, name: &str) -> SciName {
        let mut pp = peg::ParseProcessor::new();
        match peg::ParseEngine::ast(name) {
            Some(pair) => pp.ast_sci_name(pair, &name, self.is_test),
            None => pp.default_sci_name(&name, self.is_test),
        }
    }

    pub fn parse_and_format(&mut self, name: &str) -> String {
        let sn = self.parse(name);
        match &self.format {
            Format::Compact => format!("{}", serde_json::to_string(&sn).unwrap()),
            Format::Pretty => format!("{}", serde_json::to_string_pretty(&sn).unwrap()),
            Format::Simple => output::simple::row(sn).unwrap(),
        }
    }
}

#[test]
fn it_makes_gnp() {
    let mut gnp = GNParser::new();
    gnp.format = Format::Simple;
    let parsed = gnp.parse("Betula");
    assert_eq!(parsed.verbatim, "Betula");
}

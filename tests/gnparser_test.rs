use gnparser::peg::ParseEngine;
use gnparser::{Format, GNParser};
use std::fs;
use std::io::{self, BufRead};

#[test]
fn gnparser_collects_test_data() {
    let data = read_test_data();
    assert!(data.len() > 0);
}

#[test]
fn gnparser_parses_data() {
    let data = read_test_data();
    let mut gnp = GNParser::new();
    gnp.is_test = true;
    for d in &data {
        let mut parsed = gnp.parse_and_format(&d.verbatim);
        assert_eq!(parsed, d.json);
        gnp.format = Format::Simple;
        parsed = gnp.parse_and_format(&d.verbatim);
        assert_eq!(parsed.trim(), d.csv);
        parsed = ParseEngine::parsed_name(&d.verbatim);
        assert_eq!(parsed, d.raw);
    }
}

#[derive(Debug)]
struct TestData {
    verbatim: String,
    raw: String,
    json: String,
    csv: String,
}

fn read_test_data() -> Vec<TestData> {
    let f = fs::File::open("testdata/test_data.txt").expect("cannot open test_data.txt file");
    let lines = io::BufReader::new(f).lines();
    let mut item = Vec::<String>::with_capacity(4);
    let mut res = Vec::<TestData>::new();
    for l in lines {
        if let Ok(txt) = l {
            let txt1 = txt.trim().to_string();
            if txt1 == "__END__".to_string() {
                break;
            }
            if txt1.is_empty() || txt1[0..1] == "#".to_string() {
                continue;
            }
            item.push(txt1);
            if item.len() == 4 {
                let csv = item.pop().unwrap().to_string();
                let json = item.pop().unwrap().to_string();
                let raw = item.pop().unwrap().to_string();
                let verbatim = item.pop().unwrap().to_string();
                let td = TestData {
                    verbatim,
                    raw,
                    json,
                    csv,
                };
                res.push(td);
            }
        }
    }
    res
}

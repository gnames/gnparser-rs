ffn print_ast(pair: &pest::iterators::Pair<'_, Rule>, indent: usize) {
    let inner_pairs = pair.clone().into_inner();
    if inner_pairs.clone().count() > 0 {
        println!("{:indent$}{:#?}", "", pair.as_rule(), indent = indent);
        for inner_pair in inner_pairs {
            print_ast(&inner_pair, indent + 2);
        }
    }else{
        println!("{:indent$}{:#?}: {}", "", pair.as_rule(), pair.as_span().as_str(), indent = indent);
    }
}n print_ast(pair: &pest::iterators::Pair<'_, Rule>, indent: usize) {
    let inner_pairs = pair.clone().into_inner();
    if inner_pairs.clone().count() > 0 {
        println!("{:indent$}{:#?}", "", pair.as_rule(), indent = indent);
        for inner_pair in inner_pairs {
            print_ast(&inner_pair, indent + 2);
        }
    }else{
        println!("{:indent$}{:#?}: {}", "", pair.as_rule(), pair.as_span().as_str(), indent = indent);
    }
}use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "peg/gnparser.pest"]
pub struct ParseEngine;

impl ParseEngine {
    pub fn ast(name: &str) -> Option<Pair<Rule>> {
        match Self::parse(Rule::SciName, name) {
            Ok(mut parsed) => return Some(parsed.next().unwrap()),
            Err(_) => return None,
        };
    }

    pub fn parsed_name(name: &str) -> String {
        let parsed = ParseEngine::ast(name);
        match parsed {
            Some(pair) => {
                for p in pair.into_inner() {
                    match p.as_rule() {
                        Rule::SingleName => return p.as_str().to_string(),
                        _ => continue,
                    }
                }
            }
            None => return "noparse".to_string(),
        }
        "noparse".to_string()
    }
}

#[test]
fn parse_engine_parses() {
    let ast = ParseEngine::ast("Pomatomus").unwrap();
    assert_eq!(ast.as_rule(), Rule::SciName);
}

#[test]
fn parse_engine_gets_name() {
    let mut name = ParseEngine::parsed_name("Pomatomus");
    assert_eq!(name, "Pomatomus".to_string());
    name = ParseEngine::parsed_name("pomatomus");
    assert_eq!(name, "noparse");
}

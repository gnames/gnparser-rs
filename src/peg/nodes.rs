use super::sci_name::*;
use super::word_type::WordType;
use super::Rule;
use lazy_static;
use pest::iterators::Pair;
use uuid::Uuid;

lazy_static! {
    static ref GN_NAMESPACE: Uuid =
        Uuid::new_v5(&Uuid::NAMESPACE_DNS, "globalnames.org".as_bytes());
}

#[derive(Debug, Default)]
pub struct ParseProcessor {
    sn: SciName,
    pos: Vec<Pos>,
    can_full: Vec<String>,
    can_simp: Vec<String>,
    can_stem: Vec<String>,
    norm: Vec<String>,
}

impl ParseProcessor {
    pub fn new() -> Self {
        ParseProcessor {
            ..Default::default()
        }
    }

    pub fn ast_sci_name(&mut self, sci_name: Pair<Rule>, verbatim: &str, is_test: bool) -> SciName {
        for pair in sci_name.into_inner() {
            match pair.as_rule() {
                Rule::SingleName => {
                    let det = pair.into_inner().next().unwrap();
                    let details = self.new_details(det);
                    self.sn.details = Some(vec![details]);
                }
                Rule::Tail => {
                    let tail = pair.as_str().to_string();
                    if tail.len() > 0 {
                        self.sn.tail = Some(tail);
                    }
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        self.sn.verbatim = verbatim.to_string();
        self.sn.parsed = true;
        self.sn.name_string_id = name_id(verbatim);
        self.set_version(is_test);
        self.set_quality();
        self.set_canonical();
        self.set_normalized();
        self.set_pos();
        self.sn.clone()
    }

    pub fn default_sci_name(&mut self, verbatim: &str, is_test: bool) -> SciName {
        self.sn.verbatim = verbatim.to_string();
        self.set_version(is_test);
        self.sn.clone()
    }

    fn new_details(&mut self, det: Pair<Rule>) -> Details {
        match det.as_rule() {
            Rule::Uninomial => {
                let uninomial = self.new_uninomial(det);
                Details::Uninomial(uninomial)
            }
            _ => unreachable!(),
        }
    }

    fn new_uninomial(&mut self, uni: Pair<Rule>) -> Uninomial {
        let mut uninomial = Uninomial {
            ..Default::default()
        };
        for pair in uni.into_inner() {
            match pair.as_rule() {
                Rule::UninomialWord => {
                    let val = pair.as_str().to_string();
                    uninomial.value = val.clone();
                    self.can_full.push(val.clone());
                    self.can_simp.push(val.clone());
                    self.can_stem.push(val.clone());
                    self.norm.push(val.clone());
                    let sp = pair.as_span();
                    let pos = Pos(WordType::UninomialType.to_string(), sp.start(), sp.end());
                    self.pos.push(pos);
                }
                Rule::Authorship => {
                    uninomial.authorship = Some(self.new_authorship(pair));
                }
                _ => unreachable!(),
            }
        }
        uninomial
    }

    fn new_authorship(&mut self, au: Pair<Rule>) -> Authorship {
        let mut authorship = Authorship {
            ..Default::default()
        };
        for pair in au.into_inner() {
            match pair.as_rule() {
                Rule::AuthorshipCombo => {}
                Rule::OriginalAuthorship => {
                    authorship.value = pair.as_str().to_string();
                    let ag = pair.into_inner().next().unwrap();
                    authorship.basionym_authorship = Some(self.new_auth_group(ag));
                }
                _ => unreachable!(),
            }
        }
        authorship
    }

    fn new_auth_group(&mut self, ag: Pair<Rule>) -> AuthGroup {
        let mut auth_group = AuthGroup {
            ..Default::default()
        };
        let at = ag.into_inner().next().unwrap();
        let mut authors: Vec<String> = Vec::new();
        for pair in at.into_inner() {
            match pair.as_rule() {
                Rule::Author => {
                    authors.push(pair.as_str().to_string());
                    self.author_words(pair);
                }
                Rule::Year => {
                    auth_group.year = Some(Year {
                        year: pair.as_str().to_string(),
                        approximate: false,
                    })
                }
                _ => unreachable!(),
            }
        }
        auth_group.authors = authors;
        auth_group
    }

    fn author_words(&mut self, aws: Pair<Rule>) {
        for pair in aws.into_inner() {
            match pair.as_rule() {
                Rule::AuthorWord => {
                    let sp = pair.as_span();
                    let pos = Pos(WordType::AuthorWordType.to_string(), sp.start(), sp.end());
                    self.pos.push(pos);
                }
                _ => unreachable!(),
            }
        }
    }

    fn set_canonical(&mut self) {
        let can = Canonical {
            full: self.can_full.join(" "),
            simple: self.can_simp.join(" "),
            stem: self.can_stem.join(" "),
        };
        self.sn.canonical_name = Some(can);
    }

    fn set_normalized(&mut self) {
        self.sn.normalized = Some(self.norm.join(" "));
    }

    fn set_quality(&mut self) {
        match self.sn.warnings {
            Some(_) => self.sn.quality = 2,
            None => self.sn.quality = 1,
        };
    }

    fn set_version(&mut self, is_test: bool) {
        let mut ver = env!("CARGO_PKG_VERSION").to_string();
        if is_test {
            ver = "test_version".to_string();
        }
        self.sn.parser_version = ver;
    }

    fn set_pos(&mut self) {
        self.sn.positions = Some(self.pos.clone());
    }
}

fn name_id(name: &str) -> String {
    let id = Uuid::new_v5(&GN_NAMESPACE, name.as_bytes());
    id.to_string()
}

#[test]
fn correct_uuid_v5() {
    assert_eq!(name_id("Homo"), "89f48cba-d38b-5640-99ba-8dac0dcaf2f8")
}

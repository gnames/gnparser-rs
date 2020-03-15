use super::norm;
use super::sci_name::*;
use super::word_type::WordType;
use super::Rule;
use super::Warn;
use lazy_static;
use pest::iterators::Pair;
use std::cmp::Ordering;
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
                        self.sn.quality_warnings.push(Warn::TailWarn.as_warning());
                        self.sn.unparsed_tail = Some(tail);
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
        self.sort_warnings();
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
                    let (val, warns) = norm::normalize(pair.as_str());
                    if warns.len() > 0 {
                        self.sn.quality_warnings.extend_from_slice(&warns);
                    }
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
        if let Some((au, yr)) = uninomial.last_authorship() {
            self.sn.authorship = Some(au);
            self.sn.year = yr;
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
                    let ag = pair.into_inner().next().unwrap();
                    authorship.basionym_authorship = Some(self.new_auth_group(ag));
                }
                _ => unreachable!(),
            }
        }
        let (au_val, yr) = authorship_value(&authorship);
        authorship.value = au_val;
        authorship.year = yr;
        self.norm.push(authorship.value.clone());
        authorship
    }

    fn new_auth_group(&mut self, ag: Pair<Rule>) -> AuthorsGroup {
        let mut auth_group = AuthorsGroup {
            ..Default::default()
        };
        let at = ag.into_inner().next().unwrap();
        for pair in at.into_inner() {
            match pair.as_rule() {
                Rule::Author => {
                    auth_group = self.author_words(pair, auth_group);
                }
                Rule::Year => {
                    let sp = pair.as_span();
                    let p = Pos(WordType::YearType.to_string(), sp.start(), sp.end());
                    self.pos.push(p);
                    auth_group.year = Some(Year {
                        value: pair.as_str().to_string(),
                        approximate: false,
                    })
                }
                _ => unreachable!(),
            }
        }
        auth_group
    }

    fn author_words(&mut self, aws: Pair<Rule>, mut ag: AuthorsGroup) -> AuthorsGroup {
        let mut wrd: Vec<&str> = Vec::new();
        for pair in aws.into_inner() {
            match pair.as_rule() {
                Rule::AuthorWord => {
                    wrd.push(pair.as_str());
                    let sp = pair.as_span();
                    let pos = Pos(WordType::AuthorWordType.to_string(), sp.start(), sp.end());
                    self.pos.push(pos);
                }
                _ => unreachable!(),
            }
        }
        ag.authors.push(wrd.join(" "));
        ag
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
        match self.sn.quality_warnings.len() {
            0 => self.sn.quality = 1,
            _ => {
                let mut quality = 1_i8;
                for w in &self.sn.quality_warnings {
                    if w.0 > quality {
                        quality = w.0;
                    }
                }
                self.sn.quality = quality
            }
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
        let mut res: Vec<Pos> = Vec::new();
        let verb = self.sn.verbatim.clone();
        for p in &self.pos {
            let st = verb[..(p.1)].chars().count();
            let l = verb[(p.1)..(p.2)].chars().count();
            res.push(Pos(p.0.clone(), st, st + l));
        }
        self.sn.positions = Some(res);
    }

    fn sort_warnings(&mut self) {
        self.sn
            .quality_warnings
            .sort_by(|a, b| match (&b.0).cmp(&a.0) {
                Ordering::Equal => (&a.1).cmp(&b.1),
                other => other,
            });
    }
}

fn authorship_value(au: &Authorship) -> (String, Option<String>) {
    let mut val = "".to_string();
    let mut year: Option<String> = None;
    if let Some(ref ba) = au.basionym_authorship {
        let (ba_val, yr) = auth_value(&ba);
        val = format!("{}", ba_val);
        year = yr;
    }
    if let Some(ref ca) = au.combination_authorship {
        let (ca_val, _) = auth_value(&ca);
        val = format!("({}) {}", val, ca_val);
    }

    (val, year)
}

fn auth_value(ag: &AuthorsGroup) -> (String, Option<String>) {
    let aus = &ag.authors;
    let mut year: Option<String> = None;
    let mut val = match aus.len() {
        0 => unreachable!(),
        1 => aus[0].clone(),
        2 => format!("{} & {}", aus[0], aus[1]),
        _ => format!(
            "{} & {}",
            aus[0..(aus.len() - 1)].join(", "),
            aus[aus.len() - 1]
        ),
    };
    if let Some(ref yr) = ag.year {
        val = format!("{} {}", val, yr.value);
        year = Some(yr.value.clone());
    }

    (val.to_string(), year)
}

fn name_id(name: &str) -> String {
    let id = Uuid::new_v5(&GN_NAMESPACE, name.as_bytes());
    id.to_string()
}

#[test]
fn correct_uuid_v5() {
    assert_eq!(name_id("Homo"), "89f48cba-d38b-5640-99ba-8dac0dcaf2f8")
}

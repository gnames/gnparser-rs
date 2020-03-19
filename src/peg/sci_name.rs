use serde::Serialize;

/// NameAuthors trait finds the name authorship string and year if they
/// exist
pub trait NameAuthors {
    fn last_authorship(&self) -> Option<(String, Option<String>)>;
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SciName {
    pub parsed: bool,
    pub quality: i8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quality_warnings: Vec<Warning>,
    pub verbatim: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<Canonical>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorship: Option<String>,
    #[serde(skip_serializing)]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<Details>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Pos>>,
    pub surrogate: bool,
    pub virus: bool,
    pub hybrid: bool,
    pub bacteria: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unparsed_tail: Option<String>,
    pub name_string_id: String,
    pub parser_version: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Warning(pub i8, pub &'static str);

#[derive(Serialize, Debug, Clone, Default)]
pub struct Canonical {
    pub full: String,
    pub simple: String,
    pub stem: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Pos(pub String, pub usize, pub usize);

#[derive(Serialize, Clone, Debug)]
pub enum Details {
    #[serde(rename = "uninomial")]
    Uninomial(Uninomial),
}

impl Details {
    pub fn canonical(&self) -> Canonical {
        match self {
            Details::Uninomial(uni) => uni.canonical(),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Uninomial {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorship: Option<Authorship>,
}

impl Uninomial {
    fn canonical(&self) -> Canonical {
        match &self.parent {
            Some(prnt) => {
                let mut rank = "".to_string();
                if let Some(rnk) = &self.rank {
                    rank = rnk.clone();
                }
                Canonical {
                    full: format!("{} {} {}", prnt, rank, self.value),
                    simple: self.value.clone(),
                    stem: self.value.clone(),
                }
            }
            _ => Canonical {
                full: self.value.clone(),
                simple: self.value.clone(),
                stem: self.value.clone(),
            },
        }
    }
}

impl NameAuthors for Uninomial {
    fn last_authorship(&self) -> Option<(String, Option<String>)> {
        match &self.authorship {
            None => None,
            Some(au) => Some((au.value.clone(), au.year.clone())),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorship {
    pub value: String,
    #[serde(skip_serializing)]
    pub year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basionym_authorship: Option<AuthorsGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combination_authorship: Option<AuthorsGroup>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthorsGroup {
    pub authors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Year>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ex_authors: Option<Authors>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emend_authors: Option<Authors>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Authors {
    pub authors: Vec<String>,
    pub year: Option<Year>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Year {
    pub value: String,
    #[serde(skip_serializing_if = "is_false")]
    pub approximate: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

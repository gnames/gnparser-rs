use serde::Serialize;

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SciName {
    pub parsed: bool,
    pub quality: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
    pub verbatim: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_name: Option<Canonical>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorship: Option<String>,
    #[serde(skip_serializing)]
    pub year: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<Details>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Pos>>,
    pub surrogate: bool,
    pub virus: bool,
    pub hybrid: bool,
    pub bacteria: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    pub name_string_id: String,
    pub parser_version: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Warning(pub i8, pub String);

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

#[derive(Serialize, Debug, Clone, Default)]
pub struct Uninomial {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorship: Option<Authorship>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Authorship {
    pub value: String,
    pub basionym_authorship: Option<AuthGroup>,
    pub combination_authorship: Option<AuthGroup>,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthGroup {
    pub authors: Vec<String>,
    pub year: Option<Year>,
    pub ex_authors: Option<Authors>,
    pub emend_authors: Option<Authors>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Authors {
    pub authors: Vec<String>,
    pub year: Option<Year>,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Year {
    pub year: String,
    pub approximate: bool,
}

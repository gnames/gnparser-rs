use std::fmt;

#[derive(Debug)]
pub enum WordType {
    UnknownType,
    ComparisonType,
    ApproxType,
    AuthorWordType,
    AuthorWordExType,
    AuthorWordEmendType,
    AuthorWordFiliusType,
    GenusType,
    InfraSpEpithetType,
    HybridCharType,
    RankType,
    RankUniType,
    SpEpithetType,
    SubGenusType,
    SuperSpType,
    UninomialType,
    YearApproximateType,
    YearType,
}

impl fmt::Display for WordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordType::UnknownType => write!(f, "word"),
            WordType::ComparisonType => write!(f, "annotationIdentification"),
            WordType::ApproxType => write!(f, "annotationIdentification"),
            WordType::AuthorWordType => write!(f, "authorWord"),
            WordType::AuthorWordExType => write!(f, "authorWord"),
            WordType::AuthorWordEmendType => write!(f, "authorWord"),
            WordType::AuthorWordFiliusType => write!(f, "authorWordFilius"),
            WordType::GenusType => write!(f, "genus"),
            WordType::InfraSpEpithetType => write!(f, "hybridChar"),
            WordType::HybridCharType => write!(f, "infraspecificEpithet"),
            WordType::RankType => write!(f, "rank"),
            WordType::RankUniType => write!(f, "rank"),
            WordType::SpEpithetType => write!(f, "specificEpithet"),
            WordType::SubGenusType => write!(f, "infragenericEpithet"),
            WordType::SuperSpType => write!(f, "superspecies"),
            WordType::UninomialType => write!(f, "uninomial"),
            WordType::YearApproximateType => write!(f, "approximateYear"),
            WordType::YearType => write!(f, "year"),
        }
    }
}

#[test]
fn it_converts_to_sting() {
    let w = WordType::RankType;
    assert_eq!(w.to_string(), "rank".to_string());
}

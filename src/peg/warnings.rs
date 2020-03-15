use super::Warning;
use std::collections::HashMap;

lazy_static! {
    static ref WARNS: HashMap<Warn, Warning> = [
        (Warn::TailWarn, Warning(3, "Unparsed tail",)),
        (
            Warn::ApostrOtherWarn,
            Warning(3, "Not an ASCII apostrophe",)
        ),
        (
            Warn::AuthAmbiguousFiliusWarn,
            Warning(2, "Ambiguous f. (filius or forma)",)
        ),
        (
            Warn::AuthDoubleParensWarn,
            Warning(3, "Authorship in double parentheses",)
        ),
        (Warn::AuthExWarn, Warning(2, "Ex authors are not required",)),
        (Warn::AuthExWithDotWarn, Warning(3, "`ex` ends with dot",)),
        (
            Warn::AuthEmendWarn,
            Warning(2, "Emend authors are not required",)
        ),
        (
            Warn::AuthEmendWithoutDotWarn,
            Warning(3, "`emend` without a period",)
        ),
        (
            Warn::AuthMissingOneParensWarn,
            Warning(3, "Authorship is missing one parenthesis",)
        ),
        (
            Warn::AuthQuestionWarn,
            Warning(3, "Author as a question mark",)
        ),
        (Warn::AuthShortWarn, Warning(3, "Author is too short",)),
        (Warn::AuthUnknownWarn, Warning(2, "Author is unknown",)),
        (Warn::AuthUpperCaseWarn, Warning(2, "Author in upper case",)),
        (
            Warn::BacteriaMaybeWarn,
            Warning(1, "The genus is a homonym of a bacterial genus",)
        ),
        (
            Warn::BotanyAuthorNotSubgenWarn,
            Warning(2, "Possible ICN author instead of subgenus",)
        ),
        (
            Warn::CanonicalApostropheWarn,
            Warning(3, "Apostrophe is not allowed in canonical",)
        ),
        (
            Warn::CapWordQuestionWarn,
            Warning(3, "Uninomial word with question mark",)
        ),
        (
            Warn::CharBadWarn,
            Warning(2, "Non-standard characters in canonical",)
        ),
        (
            Warn::GenusAbbrWarn,
            Warning(3, "Abbreviated uninomial word",)
        ),
        (
            Warn::GenusUpperCharAfterDash,
            Warning(2, "Apparent genus with capital character after hyphen",)
        ),
        (
            Warn::GreekLetterInRank,
            Warning(2, "Deprecated Greek letter enumeration in rank",)
        ),
        (
            Warn::HTMLTagsEntitiesWarn,
            Warning(3, "HTML tags or entities in the name",)
        ),
        (
            Warn::HybridCharNoSpaceWarn,
            Warning(3, "Hybrid char not separated by space",)
        ),
        (Warn::HybridFormulaWarn, Warning(2, "Hybrid formula",)),
        (
            Warn::HybridFormulaIncompleteWarn,
            Warning(3, "Incomplete hybrid formula",)
        ),
        (
            Warn::HybridFormulaProbIncompleteWarn,
            Warning(2, "Probably incomplete hybrid formula",)
        ),
        (Warn::HybridNamedWarn, Warning(2, "Named hybrid",)),
        (Warn::NameApproxWarn, Warning(3, "Name is approximate",)),
        (Warn::NameComparisonWarn, Warning(3, "Name comparison",)),
        (Warn::RankUncommonWarn, Warning(3, "Uncommon rank",)),
        (
            Warn::SpaceMultipleWarn,
            Warning(2, "Multiple adjacent space characters",)
        ),
        (
            Warn::SpaceNonStandardWarn,
            Warning(3, "Non-standard space characters",)
        ),
        (Warn::SpeciesNumericWarn, Warning(3, "Numeric prefix",)),
        (
            Warn::SuperSpeciesWarn,
            Warning(2, "Ambiguity, subgenus or superspecies found",)
        ),
        (
            Warn::UTF8ConvBadWarn,
            Warning(3, "Incorrect conversion to UTF-8",)
        ),
        (
            Warn::UninomialComboWarn,
            Warning(2, "Combination of two uninomials",)
        ),
        (
            Warn::WhiteSpaceTrailWarn,
            Warning(2, "Trailing whitespace",)
        ),
        (Warn::YearCharWarn, Warning(2, "Year with latin character",)),
        (Warn::YearDotWarn, Warning(2, "Year with period",)),
        (
            Warn::YearOrigMisplacedWarn,
            Warning(2, "Misplaced basionym year",)
        ),
        (Warn::YearPageWarn, Warning(3, "Year with page info",)),
        (Warn::YearParensWarn, Warning(2, "Year with parentheses",)),
        (
            Warn::YearQuestionWarn,
            Warning(2, "Year with question mark",)
        ),
        (Warn::YearRangeWarn, Warning(3, "Years range",)),
        (
            Warn::YearSqBraketsWarn,
            Warning(3, "Year with square brakets",)
        ),
    ]
    .iter()
    .cloned()
    .collect();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Warn {
    TailWarn,
    ApostrOtherWarn,
    AuthAmbiguousFiliusWarn,
    AuthDoubleParensWarn,
    AuthExWarn,
    AuthExWithDotWarn,
    AuthEmendWarn,
    AuthEmendWithoutDotWarn,
    AuthMissingOneParensWarn,
    AuthQuestionWarn,
    AuthShortWarn,
    AuthUnknownWarn,
    AuthUpperCaseWarn,
    BacteriaMaybeWarn,
    BotanyAuthorNotSubgenWarn,
    CanonicalApostropheWarn,
    CapWordQuestionWarn,
    CharBadWarn,
    GenusAbbrWarn,
    GenusUpperCharAfterDash,
    GreekLetterInRank,
    HTMLTagsEntitiesWarn,
    HybridCharNoSpaceWarn,
    HybridFormulaWarn,
    HybridFormulaIncompleteWarn,
    HybridFormulaProbIncompleteWarn,
    HybridNamedWarn,
    NameApproxWarn,
    NameComparisonWarn,
    RankUncommonWarn,
    SpaceMultipleWarn,
    SpaceNonStandardWarn,
    SpeciesNumericWarn,
    SuperSpeciesWarn,
    UTF8ConvBadWarn,
    UninomialComboWarn,
    WhiteSpaceTrailWarn,
    YearCharWarn,
    YearDotWarn,
    YearOrigMisplacedWarn,
    YearPageWarn,
    YearParensWarn,
    YearQuestionWarn,
    YearRangeWarn,
    YearSqBraketsWarn,
}

impl Warn {
    pub fn as_warning(&self) -> Warning {
        WARNS.get(&self).unwrap().clone()
    }
}

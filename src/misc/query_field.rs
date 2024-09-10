use serde::Serialize;
use crate::compound_query::{
    bool::Bool,
    boosting::Boosting,
    constant_score::ConstantScore,
    disjunction_max::DisMax
};
use crate::full_text:: {
    match_boolean_prefix::MatchBoolPrefix,
    match_phrase::MatchPhrase,
    match_phrase_prefix::MatchPhrasePrefix,
    multi_match::MultiMatch,
    query_string::QueryString,
    r#match::Match,
    simple_query_string::SimpleQueryString
};
use crate::term::{
    term::Term,
    terms::Terms,
    terms_set::TermsSet,
    exists::Exists,
    fuzzy::Fuzzy,
    ids::IDs,
    prefix::Prefix,
    wildcard::Wildcard,
    regexp::Regexp,
    range::Range
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryField {
    /// https://opensearch.org/docs/latest/query-dsl/full-text/match/
    Match(Match),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/multi-match/
    MultiMatch(MultiMatch),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/query-string/
    QueryString(QueryString),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/simple-query-string/
    SimpleQueryString(SimpleQueryString),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/match-phrase/
    MatchPhrase(MatchPhrase),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/match-phrase-prefix/
    MatchPhrasePrefix(MatchPhrasePrefix),
    /// https://opensearch.org/docs/latest/query-dsl/full-text/match-bool-prefix/
    MatchBoolPrefix(MatchBoolPrefix),
    /// https://opensearch.org/docs/latest/query-dsl/term/terms/
    /// https://opensearch.org/docs/latest/query-dsl/term/terms/#terms-lookup
    Terms(Terms),
    /// https://opensearch.org/docs/latest/query-dsl/term/term/
    Term(Term),
    /// https://opensearch.org/docs/latest/query-dsl/term/terms-set/
    TermsSet(TermsSet),
    /// https://opensearch.org/docs/latest/query-dsl/compound/bool/
    Bool(Bool),
    /// https://opensearch.org/docs/latest/query-dsl/compound/boosting/
    Boosting(Boosting),
    /// https://opensearch.org/docs/latest/query-dsl/compound/constant-score/
    ConstantScore(ConstantScore),
    /// https://opensearch.org/docs/latest/query-dsl/compound/disjunction-max/
    DisMax(DisMax),
    /// https://opensearch.org/docs/latest/query-dsl/term/wildcard/
    Wildcard(Wildcard),
    /// https://opensearch.org/docs/latest/query-dsl/term/ids/
    #[serde(rename="ids")]
    IDs(IDs),
    /// https://opensearch.org/docs/latest/query-dsl/term/ids/
    Fuzzy(Fuzzy),
    /// https://opensearch.org/docs/latest/query-dsl/term/prefix/
    Prefix(Prefix),
    /// https://opensearch.org/docs/latest/query-dsl/term/regexp/
    Regexp(Regexp),
    ///https://opensearch.org/docs/latest/query-dsl/term/exists/
    Exists(Exists),
    /// https://opensearch.org/docs/latest/query-dsl/term/range/
    Range(Range),
}

macro_rules! from_types {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for QueryField {
                fn from(val: $ty) -> Self {
                    Self::$ty(val.into())
                }
            }
        )*
    }
}

from_types! {
    Match,
    MultiMatch,
    QueryString,
    SimpleQueryString,
    MatchPhrase,
    MatchPhrasePrefix,
    MatchBoolPrefix,
    Terms,
    Term,
    TermsSet,
    Bool,
    Boosting,
    ConstantScore,
    DisMax,
    Wildcard,
    IDs,
    Fuzzy,
    Prefix,
    Regexp,
    Exists,
    Range
}
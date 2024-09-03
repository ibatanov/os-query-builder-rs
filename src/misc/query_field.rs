use serde::Serialize;
use crate::compound_query::{
    bool::Bool,
    boosting::Boosting,
    constant_score::ConstantScore,
    disjunction_max::DisMax
};
use crate::full_text::match_phrase::MatchPhrase;
use crate::full_text::match_phrase_prefix::MatchPhrasePrefix;
use crate::full_text::multi_match::MultiMatch;
use crate::full_text::query_string::QueryString;
use crate::full_text::r#match::Match;
use crate::full_text::simple_query_string::SimpleQueryString;
use crate::term::{term::Term, terms::Terms};


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
    /// https://opensearch.org/docs/latest/query-dsl/term/terms/
    /// https://opensearch.org/docs/latest/query-dsl/term/terms/#terms-lookup
    Terms(Terms),
    /// https://opensearch.org/docs/latest/query-dsl/term/term/
    Term(Term),
    /// https://opensearch.org/docs/latest/query-dsl/compound/bool/
    Bool(Bool),
    /// https://opensearch.org/docs/latest/query-dsl/compound/boosting/
    Boosting(Boosting),
    /// https://opensearch.org/docs/latest/query-dsl/compound/constant-score/
    ConstantScore(ConstantScore),
    /// https://opensearch.org/docs/latest/query-dsl/compound/disjunction-max/
    DisMax(DisMax)
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
    Terms,
    Term,
    Bool,
    Boosting,
    ConstantScore,
    DisMax
}
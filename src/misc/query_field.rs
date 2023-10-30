use serde::Serialize;
use crate::full_text::match_phrase::MatchPhrase;
use crate::full_text::match_phrase_prefix::MatchPhrasePrefix;
use crate::full_text::multi_match::MultiMatch;
use crate::full_text::query_string::QueryString;
use crate::full_text::r#match::Match;
use crate::full_text::simple_query_string::SimpleQueryString;


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryField {
    Match(Match),
    MultiMatch(MultiMatch),
    QueryString(QueryString),
    SimpleQueryString(SimpleQueryString),
    MatchPhrase(MatchPhrase),
    MatchPhrasePrefix(MatchPhrasePrefix),
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
    MatchPhrasePrefix
}
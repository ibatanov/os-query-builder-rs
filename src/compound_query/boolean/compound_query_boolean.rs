use serde::Serialize;
use crate::{term::terms::Terms, term::term::Term};
use crate::compound_query::boolean::bool::Bool;
use crate::full_text::{
    r#match::Match,
    multi_match::MultiMatch,
    match_phrase::MatchPhrase,
    match_phrase_prefix::MatchPhrasePrefix,
    simple_query_string::SimpleQueryString
};


#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompoundQueryBoolean {
    Match(Match),
    MultiMatch(MultiMatch),
    MatchPhrase(MatchPhrase),
    MatchPhrasePrefix(MatchPhrasePrefix),
    SimpleQueryString(SimpleQueryString),
    Term(Term),
    Terms(Terms),
    Bool(Bool)
}

macro_rules! from_types {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for CompoundQueryBoolean {
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
    MatchPhrase,
    MatchPhrasePrefix,
    SimpleQueryString,
    Term,
    Terms, 
    Bool
}
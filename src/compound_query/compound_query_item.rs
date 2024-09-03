// use serde::Serialize;
// use crate::term::{
//     terms::Terms,
//     term::Term
// };
// use crate::compound_query::{
//     bool::Bool,
//     boost::Boosting,
//     constant_score::ConstantScore,
//     disjunction_max::DisjunctionMax
// };
// use crate::full_text::{
//     r#match::Match,
//     multi_match::MultiMatch,
//     match_phrase::MatchPhrase,
//     match_phrase_prefix::MatchPhrasePrefix,
//     simple_query_string::SimpleQueryString
// };


// #[derive(Debug, Clone, Serialize)]
// #[serde(rename_all = "snake_case")]
// pub enum CompoundQueryItem {
//     Match(Match),
//     MultiMatch(MultiMatch),
//     MatchPhrase(MatchPhrase),
//     MatchPhrasePrefix(MatchPhrasePrefix),
//     SimpleQueryString(SimpleQueryString),
//     Term(Term),
//     Terms(Terms),
//     Bool(Bool),
//     Boosting(Boosting),
//     ConstantScore(ConstantScore),
//     DisjunctionMax(DisjunctionMax)
// }
//
// macro_rules! from_types {
//     ($($ty:ident),*) => {
//         $(
//             impl From<$ty> for CompoundQueryItem {
//                 fn from(val: $ty) -> Self {
//                     Self::$ty(val.into())
//                 }
//             }
//         )*
//     }
// }
//
// from_types! {
//     Match,
//     MultiMatch,
//     MatchPhrase,
//     MatchPhrasePrefix,
//     SimpleQueryString,
//     Term,
//     Terms,
//     Bool,
//     Boosting,
//     ConstantScore,
//     DisjunctionMax
// }
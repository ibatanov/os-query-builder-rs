use serde::Serialize;

use crate::query::multi_match::MultiMatchQuery;
use crate::query::r#match::MatchQuery;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryField {
    Match(MatchQuery),
    MultiMatch(MultiMatchQuery),
}

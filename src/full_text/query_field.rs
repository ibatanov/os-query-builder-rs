use serde::Serialize;

use crate::full_text::multi_match::MultiMatchQuery;
use crate::full_text::r#match::MatchQuery;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryField {
    Match(MatchQuery),
    MultiMatch(MultiMatchQuery),
}

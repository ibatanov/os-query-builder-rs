use serde::Serialize;
use crate::full_text::intervals::rules::filter::FilterRule;

#[derive(Debug, Default, Clone, Serialize)]
pub struct MatchRule {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_gaps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Box<FilterRule>>
}

impl MatchRule {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn query<T: Into<String> + Serialize>(self, query: T) -> Self {
        Self {
            query: query.into(),
            ..self
        }
    }

    pub fn analyzer<T: Into<String> + Serialize>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn max_gaps<T: Into<i64> + Serialize>(self, max_gaps: T) -> Self {
        Self {
            max_gaps: Some(max_gaps.into()),
            ..self
        }
    }

    pub fn ordered<T: Into<bool> + Serialize>(self, ordered: T) -> Self {
        Self {
            ordered: Some(ordered.into()),
            ..self
        }
    }

    pub fn use_field<T: Into<String> + Serialize>(self, use_field: T) -> Self {
        Self {
            use_field: Some(use_field.into()),
            ..self
        }
    }

    pub fn filter<T: Into<FilterRule> + Serialize>(self, filter: T) -> Self {
        Self {
            filter: Some(Box::new(filter.into())),
            ..self
        }
    }
}

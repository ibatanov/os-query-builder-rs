use serde::Serialize;
use crate::full_text::intervals::{
    interval_rule::IntervalRule,
    rules::filter::FilterRule
};

#[derive(Debug, Default, Clone, Serialize)]
pub struct AllOfRule {
    interval: Vec<IntervalRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Box<FilterRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_gaps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ordered: Option<bool>
}

impl AllOfRule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intervals<T,F>(self, intervals: F) -> Self
        where T: Into<IntervalRule> + Serialize,
            F: IntoIterator<Item=T>
    {
        Self {
            interval: intervals.into_iter().map(|x| x.into()).collect(),
            ..self
        }
    }

    pub fn filter<T:Into<FilterRule>>(self, filter: T) -> Self {
        Self {
            filter: Some(Box::new(filter.into())),
            ..self
        }
    }

    pub fn max_gaps<T: Into<i64> + Serialize>(self, max_gaps: T) -> Self  {
        Self {
            max_gaps: Some(max_gaps.into()),
            ..self
        }
    }

    pub fn ordered<T: Into<bool> + Serialize>(self, ordered: T) -> Self  {
        Self {
            ordered: Some(ordered.into()),
            ..self
        }
    }
}

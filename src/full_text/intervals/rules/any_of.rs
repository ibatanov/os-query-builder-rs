use serde::Serialize;
use crate::full_text::intervals::{
    rules::filter::FilterRule,
    interval_rule::IntervalRule
};

#[derive(Debug, Default, Clone, Serialize)]
pub struct AnyOfRule {
    interval: Vec<IntervalRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Box<FilterRule>>
}

impl AnyOfRule{

    pub fn new() -> Self {
        Self::default()
    }

    pub fn intervals<T, F>(self, intervals: F) -> Self
        where T: Into<IntervalRule>,
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

}

use serde::Serialize;
use crate::misc::query_field::QueryField;

#[derive(Debug, Clone, Serialize)]
pub struct Boosting {
    positive: Box<QueryField>,
    negative: Box<QueryField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_boost: Option<f64>
}

impl Boosting {

    pub fn new<T,F>(positive: T, negative: F) -> Self
        where T: Into<QueryField>,
              F: Into<QueryField>
    {
        Self {
            positive: Box::new(positive.into()),
            negative: Box::new(negative.into()),
            negative_boost: None
        }
    }

    pub fn negative_boost<T: Into<f64>>(self, negative_boost: T) -> Self {
        Self {
            negative_boost: Some(negative_boost.into()),
            ..self
        }
    }
}
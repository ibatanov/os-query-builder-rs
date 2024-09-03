use serde::Serialize;
use crate::misc::query_field::QueryField;

#[derive(Debug, Clone, Serialize)]
pub struct ConstantScore {
    filter: Box<QueryField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>
}

impl ConstantScore {
    pub fn new<T:Into<QueryField>>(filter: T) -> Self {
        Self {
            filter: Box::new(filter.into()),
            boost: None
        }
    }

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }
}
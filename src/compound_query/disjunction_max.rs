use serde::Serialize;
use crate::misc::query_field::QueryField;

#[derive(Debug, Clone, Serialize)]
pub struct DisMax {
    queries: Vec<QueryField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tie_breaker: Option<f64>
}


impl DisMax {

    pub fn new<T, F>(queries: F) -> Self
        where T: Into<QueryField>,
            F: IntoIterator<Item=T>
    {
        Self {
            queries: queries.into_iter().map(|x|x.into()).collect(),
            tie_breaker: None
        }
    }

    pub fn tie_breaker<T: Into<f64>>(self, tie_breaker: T) -> Self {
        Self {
            tie_breaker: Some(tie_breaker.into()),
            ..self
        }
    }
}
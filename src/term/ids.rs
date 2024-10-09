use serde::Serialize;
use crate::term::term_type::TermType;

#[derive(Debug, Default, Clone, Serialize)]
pub struct IDs {
    values: Vec<TermType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

impl IDs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn values<T, F>(self, values: F) -> Self
        where T: Into<TermType>,
            F: IntoIterator<Item=T>
    {
        Self {
            values: values.into_iter().map(|x| x.into()).collect(),
            ..self
        }
    }

    pub fn boost<T:Into<f64> + Serialize>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }

}
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Exists {
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

impl Exists {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn field<T: Into<String> + Serialize>(self, field: T) -> Self {
        Self {
            field: field.into(),
            ..self
        }
    }

    pub fn boost<T: Into<f64> + Serialize>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }
}
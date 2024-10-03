use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Script {
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>
}


impl Script {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn source<T: Into<String> + Serialize>(self, source: T) -> Self {
        Self {
            source: source.into(),
            ..self
        }
    }

    pub fn lang<T: Into<String> + Serialize>(self, source: T) -> Self {
        Self {
            source: source.into(),
            ..self
        }
    }
}
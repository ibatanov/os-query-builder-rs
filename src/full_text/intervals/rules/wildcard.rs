use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct WildcardRule {
    pattern: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_field: Option<String>
}


impl WildcardRule {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn pattern<T: Into<String> + Serialize>(self, pattern: T) -> Self {
        Self {
            pattern: pattern.into(),
            ..self
        }
    }

    pub fn analyzer<T: Into<String> + Serialize>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn use_field<T: Into<String> + Serialize>(self, use_field: T) -> Self {
        Self {
            use_field: Some(use_field.into()),
            ..self
        }
    }
}
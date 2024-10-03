use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct PrefixRule {
    prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_field: Option<String>
}

impl PrefixRule {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix<T: Into<String> + Serialize>(self, prefix: T) -> Self {
        Self {
            prefix: prefix.into(),
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

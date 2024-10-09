use serde::Serialize;
use crate::misc::fuzziness::Fuzziness;

#[derive(Debug, Default, Clone, Serialize)]
pub struct FuzzyRule {
    term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpositions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_field: Option<String>
}


impl FuzzyRule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn term<T: Into<String> + Serialize>(self, term: T) -> Self {
        Self {
            term: term.into(),
            ..self
        }
    }

    pub fn analyzer<T: Into<String> + Serialize>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn fuzziness<T: Into<Fuzziness> + Serialize>(self, fuzziness: T) -> Self {
        Self {
            fuzziness: Some(fuzziness.into()),
            ..self
        }
    }

    pub fn transpositions<T: Into<bool> + Serialize>(self, transpositions: T) -> Self {
        Self {
            transpositions: Some(transpositions.into()),
            ..self
        }
    }

    pub fn prefix_length<T: Into<i64> + Serialize>(self, prefix_length: T) -> Self {
        Self {
            prefix_length: Some(prefix_length.into()),
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
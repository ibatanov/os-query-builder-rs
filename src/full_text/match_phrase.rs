use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct MatchPhrase {
    field: Option<String>,
    value: MatchPhraseValues,
}

impl MatchPhrase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, val: T) -> Self {
        let value = MatchPhraseValues {
            query: Some(val.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn query<T: Into<Value>>(self, value: T) -> Self {
        self.value(value)
    }

    pub fn slop<T: Into<u64>>(self, slop: T) -> Self {
        let value = MatchPhraseValues {
            slop: Some(slop.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        let value = MatchPhraseValues {
            analyzer: Some(analyzer.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn zero_terms_query<T: Into<String>>(self, zero_terms_query: T) -> Self {
        let value = MatchPhraseValues {
            zero_terms_query: Some(zero_terms_query.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
struct MatchPhraseValues {
    query: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    slop: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zero_terms_query: Option<String>,
}

impl Serialize for MatchPhrase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

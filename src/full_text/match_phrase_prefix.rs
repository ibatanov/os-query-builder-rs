use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct MatchPhrasePrefix {
    field: Option<String>,
    value: MatchPhrasePrefixValues,
}

impl MatchPhrasePrefix {
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
        let value = MatchPhrasePrefixValues {
            query: Some(val.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn query<T: Into<Value>>(self, value: T) -> Self {
        self.value(value)
    }

    pub fn slop<T: Into<u64>>(self, slop: T) -> Self {
        let value = MatchPhrasePrefixValues {
            slop: Some(slop.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        let value = MatchPhrasePrefixValues {
            analyzer: Some(analyzer.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn max_expansions<T: Into<u64>>(self, max_expansions: T) -> Self {
        let value = MatchPhrasePrefixValues {
            max_expansions: Some(max_expansions.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}

#[derive(Debug, Default, Clone, serde::Serialize)]
struct MatchPhrasePrefixValues {
    query: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    slop: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,
}

impl Serialize for MatchPhrasePrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}
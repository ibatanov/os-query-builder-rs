use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::misc::{
    fuzziness::Fuzziness,
    operator::Operator,
    rewrite::Rewrite
};

#[derive(Debug, Default, Clone)]
pub struct MatchBoolPrefix {
    field: Option<String>,
    value: MatchBoolPrefixValue
}

#[derive(Debug, Default, Clone, Serialize)]
struct MatchBoolPrefixValue {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_rewrite: Option<Rewrite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_transpositions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>
}

impl Serialize for MatchBoolPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}


impl MatchBoolPrefix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn query<T: Into<String>>(self, value: T) -> Self {
        let value = MatchBoolPrefixValue {
            query: value.into(),
            ..self.value
        };
        Self { value, ..self }
    }
    pub fn fuzzy_rewrite<T: Into<Rewrite>>(self, fuzzy_rewrite: T) -> Self {
        let value = MatchBoolPrefixValue {
            fuzzy_rewrite: Some(fuzzy_rewrite.into()),
            ..self.value
        };
        Self { value, ..self }
    }


    pub fn operator<T: Into<Operator>>(self, operator: T) -> Self {
        let value = MatchBoolPrefixValue {
            operator: Some(operator.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        let value = MatchBoolPrefixValue {
            analyzer: Some(analyzer.into()),
            ..self.value
        };
        Self { value, ..self }
    }


    pub fn prefix_length<T: Into<u64>>(self, prefix_length: T) -> Self {
        let value = MatchBoolPrefixValue {
            prefix_length: Some(prefix_length.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn max_expansions<T: Into<u64>>(self, max_expansions: T) -> Self {
        let value = MatchBoolPrefixValue {
            max_expansions: Some(max_expansions.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        let value = MatchBoolPrefixValue {
            fuzziness: Some(fuzziness.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn fuzzy_transpositions(self, fuzzy_transpositions: bool) -> Self {
        let value = MatchBoolPrefixValue {
            fuzzy_transpositions: Some(fuzzy_transpositions),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn minimum_should_match<T: Into<String>>(self, minimum_should_match: T) -> Self {
        let value = MatchBoolPrefixValue {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}
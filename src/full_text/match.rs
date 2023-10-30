use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::Value;
use crate::misc::fuzziness::Fuzziness;

use crate::misc::operator::Operator;
use crate::misc::zero_terms_query::ZeroTermsQuery;

#[derive(Debug, Default, Clone)]
pub struct Match {
    field: Option<String>,
    value: MatchValues,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct MatchValues {
    query: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zero_terms_query: Option<ZeroTermsQuery>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

impl Serialize for Match {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

impl Match {
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
        let value = MatchValues {
            query: Some(val.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn query<T: Into<Value>>(self, value: T) -> Self {
        self.value(value)
    }

    pub fn operator<T: Into<Operator>>(self, operator: T) -> Self {
        let value = MatchValues {
            operator: Some(operator.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        let value = MatchValues {
            analyzer: Some(analyzer.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn zero_terms_query<T: Into<ZeroTermsQuery>>(self, zero_terms_query: T) -> Self {
        let value = MatchValues {
            zero_terms_query: Some(zero_terms_query.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn lenient(self, lenient: bool) -> Self {
        let value = MatchValues {
            lenient: Some(lenient),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn prefix_length<T: Into<u64>>(self, prefix_length: T) -> Self {
        let value = MatchValues {
            prefix_length: Some(prefix_length.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn max_expansions<T: Into<u64>>(self, max_expansions: T) -> Self {
        let value = MatchValues {
            max_expansions: Some(max_expansions.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        let value = MatchValues {
            boost: Some(boost.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        let value = MatchValues {
            fuzziness: Some(fuzziness.into()),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn fuzzy_transpositions(self, fuzzy_transpositions: bool) -> Self {
        let value = MatchValues {
            fuzzy_transpositions: Some(fuzzy_transpositions),
            ..self.value
        };
        Self { value, ..self }
    }

    pub fn minimum_should_match<T: Into<String>>(self, minimum_should_match: T) -> Self {
        let value = MatchValues {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self.value
        };
        Self { value, ..self }
    }

}
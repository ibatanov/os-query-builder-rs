use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::Value;

use crate::misc::operator::Operator;

#[derive(Debug, Default, Clone)]
pub struct MatchQuery {
    field: String,
    value: MatchValues,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct MatchValues {
    query: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
}

impl Serialize for MatchQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field, &self.value)?;
        state.end()
    }
}

impl MatchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: field.into(),
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

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        let value = MatchValues {
            boost: Some(boost.into()),
            ..self.value
        };
        Self { value, ..self }
    }
}
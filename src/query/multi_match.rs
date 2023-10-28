use serde::Serialize;
use serde_json::Value;

use crate::misc::fuzziness::Fuzziness;
use crate::misc::operator::Operator;
use crate::misc::r#type::Type;

#[derive(Debug, Default, Clone, Serialize)]
pub struct MultiMatchQuery {
    query: Option<Value>,
    fields: Vec<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    query_type: Option<Type>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<Operator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,
}

impl MultiMatchQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        Self {
            query: Some(value.into()),
            ..self
        }
    }

    pub fn fields<F, T>(self, fields: F) -> Self
        where
            F: IntoIterator<Item=T>,
            T: Into<String>,
    {
        Self {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            ..self
        }
    }

    pub fn query_type<T: Into<Type>>(self, typ: T) -> Self {
        Self {
            query_type: Some(typ.into()),
            ..self
        }
    }

    pub fn operator<T: Into<Operator>>(self, operator: T) -> Self {
        Self {
            operator: Some(operator.into()),
            ..self
        }
    }

    pub fn minimum_should_match<T: Into<u64>>(self, minimum_should_match: T) -> Self {
        Self {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self
        }
    }

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        Self {
            fuzziness: Some(fuzziness.into()),
            ..self
        }
    }
}
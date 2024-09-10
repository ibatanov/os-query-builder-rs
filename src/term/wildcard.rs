use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::misc::rewrite::Rewrite;

#[derive(Debug, Default, Clone)]
pub struct Wildcard {
    field: Option<String>,
    value: WildcardValue
}

#[derive(Debug, Default, Clone, Serialize)]
struct WildcardValue {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    case_insensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Rewrite>
}

impl Serialize for Wildcard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

impl Wildcard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T:Into<String> + Serialize>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T:Into<String> + Serialize>(self, value: T) -> Self {
        Self {
            value: WildcardValue {
                value: value.into(),
                ..self.value
            },
            ..self
        }
    }

    pub fn boost<T:Into<f64> + Serialize>(self, boost: T) -> Self {
        Self {
            value: WildcardValue {
                boost: Some(boost.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn case_insensitive<T:Into<bool> + Serialize>(self, case_insensitive: T) -> Self {
        Self {
            value: WildcardValue {
                case_insensitive: Some(case_insensitive.into()),
                ..self.value
            },
            ..self
        }
    }


    pub fn rewrite<T:Into<Rewrite> + Serialize>(self, rewrite: T) -> Self {
        Self {
            value: WildcardValue {
                rewrite: Some(rewrite.into()),
                ..self.value
            },
            ..self
        }
    }
}
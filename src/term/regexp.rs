use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::misc::{
    rewrite::Rewrite,
    regexp_flag::RegexpFlag
};

#[derive(Debug, Default, Clone)]
pub struct Regexp {
    field: Option<String>,
    value: RegexpValue
}

#[derive(Debug, Default, Clone, Serialize)]
struct RegexpValue {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    case_insensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<RegexpFlag>, // todo что это?
    #[serde(skip_serializing_if = "Option::is_none")]
    max_determinized_states: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Rewrite>
}

impl Serialize for Regexp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}


impl Regexp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T:Into<String>+Serialize>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T:Into<String>+Serialize>(self, value: T) -> Self {
        Self {
            value: RegexpValue {
                value: value.into(),
                ..self.value
            },
            ..self
        }
    }

    pub fn boost<T:Into<f64>+Serialize>(self, boost: T) -> Self {
        Self {
            value: RegexpValue {
                boost: Some(boost.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn case_insensitive<T:Into<bool>+Serialize>(self, case_insensitive: T) -> Self {
        Self {
            value: RegexpValue {
                case_insensitive: Some(case_insensitive.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn max_determinized_states<T:Into<u64>+Serialize>(self, max_determinized_states: T) -> Self {
        Self {
            value: RegexpValue {
                max_determinized_states: Some(max_determinized_states.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn rewrite<T:Into<Rewrite>+Serialize>(self, rewrite: T) -> Self {
        Self {
            value: RegexpValue {
                rewrite: Some(rewrite.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn flags<T:Into<RegexpFlag>+Serialize>(self, flags: T) -> Self {
        Self {
            value: RegexpValue {
                flags: Some(flags.into()),
                ..self.value
            },
            ..self
        }
    }
}
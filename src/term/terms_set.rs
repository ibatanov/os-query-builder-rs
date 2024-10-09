use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use crate::misc::script::Script;

#[derive(Debug, Default, Clone)]
pub struct TermsSet {
    field: Option<String>,
    value: TermsSetValue
}
#[derive(Debug, Default, Clone, Serialize)]
pub struct TermsSetValue {

    terms: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match_script: Option<Script>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>
}

impl TermsSet {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T:Into<String> + Serialize>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn minimum_should_match_field<T:Into<String> + Serialize>(self, minimum_should_match_field: T) -> Self {
        Self {
            value: TermsSetValue {
                minimum_should_match_field: Some(minimum_should_match_field.into()),
                minimum_should_match_script: None,
                ..self.value
            },
            ..self
        }
    }

    pub fn minimum_should_match_script<T:Into<Script> + Serialize>(self, minimum_should_match_script: T) -> Self {
        Self {
            value: TermsSetValue {
                minimum_should_match_script: Some(minimum_should_match_script.into()),
                minimum_should_match_field: None,
                ..self.value
            },
            ..self
        }
    }

    pub fn terms<T, F>(self, terms: F) -> Self
        where T: Into<String> + Serialize,
              F: IntoIterator<Item=T>
    {
        Self {
            value: TermsSetValue {
                terms: terms.into_iter().map(|x| x.into()).collect(),
                ..self.value
            },
            ..self
        }
    }


    pub fn boost<T:Into<f64> + Serialize>(self, boost: T) -> Self {
        Self {
            value: TermsSetValue {
                boost: Some(boost.into()),
                ..self.value
            },
            ..self
        }
    }


}

impl Serialize for TermsSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}
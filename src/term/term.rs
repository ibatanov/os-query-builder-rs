use serde::{Serialize, Serializer, ser::SerializeMap};

#[derive(Debug, Default, Clone)]
pub struct Term {   
    field: String,
    value: TermInner,
}

#[derive(Debug, Default, Clone, Serialize)]
struct TermInner {   
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    case_insensitive: Option<bool>
}

impl Term {

    pub fn new<T,F>(field: T, value: F) -> Self 
        where T: Into<String>,
            F: Into<String>
    {
        Self {
            field: field.into(),
            value: TermInner{
                value: value.into(),
                boost: None,
                case_insensitive: None
            }
        }
    }
    
    pub fn boost<T:Into<f64>>(self, boost: T) -> Self {
        Self {
            value:  TermInner {
                boost: Some(boost.into()),
                    ..self.value
            },
            ..self
        }
    }
    
    pub fn case_insensitive<T:Into<bool>>(self, case_insensitive: T) -> Self {
        Self {
            value:  TermInner {
                case_insensitive: Some(case_insensitive.into()),
                    ..self.value
            },
            ..self
        }
    }
}

impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field, &self.value)?;
        state.end()
    }
}
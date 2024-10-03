use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::full_text::intervals::interval_rule::IntervalRule;

#[derive(Debug, Default, Clone)]
pub struct Intervals {
    field: Option<String>,
    value: Option<IntervalRule>
}

impl Serialize for Intervals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}


impl Intervals {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String> + Serialize>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn intervals<T: Into<IntervalRule> + Serialize>(self, interval: T) -> Self {
        Self {
            value: Some(interval.into()),
            ..self
        }
    }
}
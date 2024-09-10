use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::misc::relation::Relation;
use crate::term::term_type::TermType;

#[derive(Debug, Default, Clone)]
pub struct Range {
    field: Option<String>,
    value: RangeValue
}

#[derive(Debug, Default, Clone, Serialize)]
struct RangeValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<TermType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<TermType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<TermType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<TermType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relation: Option<Relation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>
}


impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

impl Range {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String> + Serialize>(self, field: T) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn gt<T: Into<TermType> + Serialize>(self, gt: T) -> Self {
        Self {
            value: RangeValue {
                gt: Some(gt.into()),
                ..self.value
            },
            ..self
        }
    }
    pub fn gte<T: Into<TermType> + Serialize>(self, gte: T) -> Self {
        Self {
            value: RangeValue {
                gte: Some(gte.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn lt<T: Into<TermType> + Serialize>(self, lt: T) -> Self {
        Self {
            value: RangeValue {
                lt: Some(lt.into()),
                ..self.value
            },
            ..self
        }
    }
    pub fn lte<T: Into<TermType> + Serialize>(self, lte: T) -> Self {
        Self {
            value: RangeValue {
                lte: Some(lte.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn time_zone<T: Into<String> + Serialize>(self, time_zone: T) -> Self {
        Self {
            value: RangeValue {
                time_zone: Some(time_zone.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn format<T: Into<String> + Serialize>(self, format: T) -> Self {
        Self {
            value: RangeValue {
              format: Some(format.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn boost<T: Into<f64> + Serialize>(self, boost: T) -> Self {
        Self {
            value: RangeValue {
                boost: Some(boost.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn relation<T: Into<Relation> + Serialize>(self, relation: T) -> Self {
        Self {
            value: RangeValue {
                relation: Some(relation.into()),
                ..self.value
            },
            ..self
        }
    }
}

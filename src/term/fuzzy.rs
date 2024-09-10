use serde::{
    Serialize,
    Serializer,
    ser::SerializeMap
};
use crate::misc::{
    fuzziness::Fuzziness,
    rewrite::Rewrite
};

#[derive(Debug, Default, Clone)]
pub struct Fuzzy {
    field: Option<String>,
    value: FuzzyValue

}

#[derive(Debug, Default, Clone, Serialize)]
struct FuzzyValue {
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_expansions: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Rewrite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpositions: Option<bool>
}

impl Serialize for Fuzzy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(1))?;
        state.serialize_entry(&self.field.as_deref().unwrap_or_default(), &self.value)?;
        state.end()
    }
}

impl Fuzzy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String> + Serialize>(self, field: T ) -> Self {
        Self {
            field: Some(field.into()),
            ..self
        }
    }

    pub fn value<T: Into<String> + Serialize>(self, value: T ) -> Self {
        Self {
            value: FuzzyValue {
                value: value.into(),
                ..self.value
            },
            ..self
        }
    }

    pub fn boost<T: Into<f64> + Serialize>(self, boost: T ) -> Self {
        Self {
            value: FuzzyValue {
                boost: Some(boost.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn fuzziness<T: Into<Fuzziness> + Serialize>(self, fuzziness: T ) -> Self {
        Self {
            value: FuzzyValue {
                fuzziness: Some(fuzziness.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn max_expansions<T: Into<u64> + Serialize>(self, max_expansions: T ) -> Self {
        Self {
            value: FuzzyValue {
                max_expansions: Some(max_expansions.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn prefix_length<T: Into<u64> + Serialize>(self, prefix_length: T ) -> Self {
        Self {
            value: FuzzyValue {
                prefix_length: Some(prefix_length.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn rewrite<T: Into<Rewrite> + Serialize>(self, rewrite: T ) -> Self {
        Self {
            value: FuzzyValue {
                rewrite: Some(rewrite.into()),
                ..self.value
            },
            ..self
        }
    }

    pub fn transpositions<T: Into<bool> + Serialize>(self, transpositions: T ) -> Self {
        Self {
            value: FuzzyValue {
                transpositions: Some(transpositions.into()),
                ..self.value
            },
            ..self
        }
    }
}
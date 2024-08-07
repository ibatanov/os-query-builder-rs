use std::collections::HashMap;
use serde::Serialize;
use super::term_type::TermType;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Terms {

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_query: Option<HashMap<String, Vec<TermType>>>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    terms_lookup: Option<HashMap<String, TermsLookup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TermsLookup {
    index: String,
    id: String,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing: Option<String>,
}

impl Terms {
    pub fn new_with_terms_query<F,T,K>(field_name: K, values: F) -> Self
        where
            F: IntoIterator<Item=T>,
            T: Into<TermType>,
            K: Into<String>
    {
        let mut terms = HashMap::with_capacity(1);
        terms.insert(field_name.into(), values.into_iter()
                                        .map(|x| x.into())
                                        .collect()
        );

        Self {
            terms_query: Some(terms),
            terms_lookup: None,
            boost: None
        }
    }

    pub fn new_with_terms_lookup<T:Into<String>>(field_name: T, values: TermsLookup) -> Self
    {

        let mut terms = HashMap::with_capacity(1);
        terms.insert(field_name.into(), values.into());

        Self {
            terms_query: None,
            terms_lookup: Some(terms),
            boost: None
        }
    }

    pub fn boost(self, boost: f64) -> Self {
        Self {
            boost: Some(boost),
            ..self
        }
    }
}


impl TermsLookup {

    pub fn new<T,F,X>(index: T, id: F, path: X) -> Self 
        where T: Into<String>,
            F: Into<String>,
            X: Into<String>
    {
        Self {
            index: index.into(),
            id: id.into(),
            path: path.into(),
            routing: None
        }
    }

    pub fn routing<T:Into<String>>(self, routing: T) -> Self {
        Self {
            routing: Some(routing.into()),
            ..self
        }
    }
}
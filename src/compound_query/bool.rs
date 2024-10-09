use serde::Serialize;
use crate::misc::query_field::QueryField;


#[derive(Debug, Clone, Serialize)]
pub struct Bool {
    #[serde(skip_serializing_if = "Option::is_none")]
    must: Option<Vec<QueryField>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    must_not: Option<Vec<QueryField>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<QueryField>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    should: Option<Vec<QueryField>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<usize>
    
}

impl Bool {
    
    pub fn new() -> Self {
        Self {
            must: None,
            must_not: None,
            should: None,
            filter: None,
            minimum_should_match: None
        }
    }
    
    pub fn must<T,F>(self, must_value: F) -> Self
        where T: Into<QueryField>,
            F: IntoIterator<Item = T>
    {
        
        Self {
            must: Some(must_value.into_iter().map(|x| x.into()).collect()),
            ..self
        }
    }
    
    pub fn must_not<T,F>(self, must_value: F) -> Self
        where T: Into<QueryField>,
            F: IntoIterator<Item = T>
    {
        
        Self {
            must_not: Some(must_value.into_iter().map(|x| x.into()).collect()),
            ..self
        }
    }
    
    pub fn should<T,F>(self, must_value: F) -> Self
        where T: Into<QueryField>,
            F: IntoIterator<Item = T>
    {
        
        Self {
            should: Some(must_value.into_iter().map(|x| x.into()).collect()),
            ..self
        }
    }
    
    pub fn filter<T,F>(self, filter_values: F) -> Self
        where T: Into<QueryField>,
            F: IntoIterator<Item = T>
    {
        
        Self {
            filter: Some(filter_values.into_iter().map(|x| x.into()).collect()),
            ..self
        }
    }
    
    pub fn minimum_should_match<T: Into<usize>>(self, minimum_should_match : T) -> Self {
        Self {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self
        }
    }
    
}
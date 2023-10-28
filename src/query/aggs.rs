use std::collections::HashMap;

use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Aggregates {
    field: String,
    value: Value,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct AggregateTermsValues {
    field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
}

impl Aggregates {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        Self {
            field: field.into(),
            ..self
        }
    }

    pub fn value<T: Into<Value>>(self, value: T) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }
}

impl Serialize for Aggregates {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut map: HashMap<&str, &Value> = HashMap::new();
        map.insert("terms", &self.value);

        let mut term = serializer.serialize_map(Some(1))?;
        term.serialize_entry(self.field.as_str(), &map)?;
        term.end()
    }
}


#[test]
fn test() {
    let terms = json!({
            "field": "brand.keyword",
            "size": 500
          });
    let aggs = Aggregates::new().field("brands")
        .value(terms);
    let json = serde_json::to_value(aggs).unwrap();
    println!("{}", json.to_string());
}
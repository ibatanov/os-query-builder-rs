use serde::Serialize;

/// https://www.elastic.co/guide/en/elasticsearch/reference/current/regexp-syntax.html#_valid_values
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RegexpFlag {
    All,
    Empty,
    Complement,
    Interval,
    Intersection,
    AnyString,
    None
}
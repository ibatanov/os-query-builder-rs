use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Relation {
    WithIn,
    Contains,
    Intersects,
}
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    BestFields,
    MostFields,
    CrossFields,
    Phrase,
    PhrasePrefix,
}
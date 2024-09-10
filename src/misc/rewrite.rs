use serde::{Serialize, Serializer};

/// all values from docs elasticsearch, exclude constant_score_blended
/// https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-multi-term-rewrite.html
#[derive(Debug, Clone)]
pub enum Rewrite {
    ConstantScore,
    ScoringBoolean,
    ConstantScoreBoolean,
    TopTerms(u64),
    TopTermsBoost(u64),
    TopTermsBlendedFreqs(u64)
}


impl Serialize for Rewrite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            Rewrite::TopTerms(u) => serializer.serialize_str(format!("top_terms_{}", *u).as_str()),
            Rewrite::TopTermsBoost(u) => serializer.serialize_str(format!("top_terms_boost_{}", *u).as_str()),
            Rewrite::TopTermsBlendedFreqs(u) => serializer.serialize_str(format!("top_terms_blended_freqs_{}", *u).as_str()),
            Rewrite::ConstantScore => serializer.serialize_str("constant_score"),
            Rewrite::ScoringBoolean => serializer.serialize_str("scoring_boolean"),
            Rewrite::ConstantScoreBoolean => serializer.serialize_str("constant_score_boolean")
        }
    }
}
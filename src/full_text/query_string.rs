use serde::Serialize;
use crate::misc::fuzziness::Fuzziness;
use crate::misc::operator::Operator;
use crate::misc::r#type::Type;

#[derive(Debug, Default, Clone, Serialize)]
pub struct QueryString {
    query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_field: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    query_type: Option<Type>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzziness: Option<Fuzziness>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_transpositions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_max_expansions: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzy_prefix_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_should_match: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_operator: Option<Operator>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lenient: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    allow_leading_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enable_position_increments: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    phrase_slop: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_determinized_states: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    quote_field_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    quote_analyzer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyze_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_generate_synonyms_phrase_query: Option<bool>,
}

impl QueryString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value<T: Into<String>>(self, value: T) -> Self {
        Self {
            query: Some(value.into()),
            ..self
        }
    }

    pub fn query<T: Into<String>>(self, query: T) -> Self {
        self.value(query)
    }

    pub fn default_field<T: Into<String>>(self, default_field: T) -> Self {
        Self {
            default_field: Some(default_field.into()),
            ..self
        }
    }

    pub fn query_type<T: Into<Type>>(self, typ: T) -> Self {
        Self {
            query_type: Some(typ.into()),
            ..self
        }
    }

    pub fn fuzziness<T: Into<Fuzziness>>(self, fuzziness: T) -> Self {
        Self {
            fuzziness: Some(fuzziness.into()),
            ..self
        }
    }

    pub fn fuzzy_transpositions(self, fuzzy_transpositions: bool) -> Self {
        Self {
            fuzzy_transpositions: Some(fuzzy_transpositions),
            ..self
        }
    }

    pub fn fuzzy_max_expansions<T: Into<u64>>(self, fuzzy_max_expansions: T) -> Self {
        Self {
            fuzzy_max_expansions: Some(fuzzy_max_expansions.into()),
            ..self
        }
    }

    pub fn fuzzy_prefix_length<T: Into<u64>>(self, fuzzy_prefix_length: T) -> Self {
        Self {
            fuzzy_prefix_length: Some(fuzzy_prefix_length.into()),
            ..self
        }
    }

    pub fn minimum_should_match<T: Into<String>>(self, minimum_should_match: T) -> Self {
        Self {
            minimum_should_match: Some(minimum_should_match.into()),
            ..self
        }
    }

    pub fn default_operator<T: Into<Operator>>(self, default_operator: T) -> Self {
        Self {
            default_operator: Some(default_operator.into()),
            ..self
        }
    }

    pub fn analyzer<T: Into<String>>(self, analyzer: T) -> Self {
        Self {
            analyzer: Some(analyzer.into()),
            ..self
        }
    }

    pub fn lenient(self, lenient: bool) -> Self {
        Self {
            lenient: Some(lenient),
            ..self
        }
    }

    pub fn boost<T: Into<f64>>(self, boost: T) -> Self {
        Self {
            boost: Some(boost.into()),
            ..self
        }
    }

    pub fn allow_leading_wildcard(self, allow_leading_wildcard: bool) -> Self {
        Self {
            allow_leading_wildcard: Some(allow_leading_wildcard),
            ..self
        }
    }

    pub fn enable_position_increments(self, enable_position_increments: bool) -> Self {
        Self {
            enable_position_increments: Some(enable_position_increments),
            ..self
        }
    }

    pub fn phrase_slop<T: Into<u64>>(self, phrase_slop: T) -> Self {
        Self {
            phrase_slop: Some(phrase_slop.into()),
            ..self
        }
    }

    pub fn max_determinized_states<T: Into<u64>>(self, max_determinized_states: T) -> Self {
        Self {
            max_determinized_states: Some(max_determinized_states.into()),
            ..self
        }
    }

    pub fn time_zone<T: Into<String>>(self, time_zone: T) -> Self {
        Self {
            time_zone: Some(time_zone.into()),
            ..self
        }
    }

    pub fn quote_field_suffix<T: Into<String>>(self, quote_field_suffix: T) -> Self {
        Self {
            quote_field_suffix: Some(quote_field_suffix.into()),
            ..self
        }
    }

    pub fn quote_analyzer<T: Into<String>>(self, quote_analyzer: T) -> Self {
        Self {
            quote_analyzer: Some(quote_analyzer.into()),
            ..self
        }
    }

    pub fn analyze_wildcard(self, analyze_wildcard: bool) -> Self {
        Self {
            analyze_wildcard: Some(analyze_wildcard),
            ..self
        }
    }

    pub fn auto_generate_synonyms_phrase_query(
        self,
        auto_generate_synonyms_phrase_query: bool,
    ) -> Self {
        Self {
            auto_generate_synonyms_phrase_query: Some(auto_generate_synonyms_phrase_query),
            ..self
        }
    }
}
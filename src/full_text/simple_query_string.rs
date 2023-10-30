use serde::Serialize;
use crate::misc::operator::Operator;

#[derive(Debug, Default, Clone, Serialize)]
pub struct SimpleQueryString {
    query: Option<String>,
    fields: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<String>,

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
    quote_field_suffix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    analyze_wildcard: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_generate_synonyms_phrase_query: Option<bool>,
}

impl SimpleQueryString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fields<F, T>(self, fields: F) -> Self
        where
            F: IntoIterator<Item = T>,
            T: Into<String>,
    {
        Self {
            fields: fields.into_iter().map(|f| f.into()).collect(),
            ..self
        }
    }

    pub fn field<T: Into<String>>(self, field: T) -> Self {
        let mut fields = self.fields;
        fields.push(field.into());
        Self { fields, ..self }
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

    pub fn flags<T: Into<String>>(self, flags: T) -> Self {
        Self {
            flags: Some(flags.into()),
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

    pub fn quote_field_suffix<T: Into<String>>(self, quote_field_suffix: T) -> Self {
        Self {
            quote_field_suffix: Some(quote_field_suffix.into()),
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
//! OpenSearch Query Builder for Rust
//!
//! This library provides a type-safe, builder-pattern API for constructing
//! OpenSearch queries. It supports various query types including term-level queries,
//! full-text queries, and compound queries.
//!
//! # Features
//!
//! - **Type-safe query building**: Compile-time checking of query structure
//! - **Builder pattern**: Fluent API for constructing complex queries
//! - **Comprehensive coverage**: Support for most OpenSearch query types
//! - **Serialization**: Automatic JSON serialization using Serde
//!
//! # Quick Start
//!
//! ```rust
//! use os_query_builder_rs::model::Query;
//! use os_query_builder_rs::term::term::Term;
//!
//! // Create a simple term query
//! let query = Query::new()
//!     .query(Term::new("field", "value"));
//!
//! // Serialize to JSON
//! let json = serde_json::to_string(&query).unwrap();
//! ```
//!
//! # Query Types
//!
//! - **Term-level queries**: `Term`, `Terms`, `Range`, `Exists`, etc.
//! - **Full-text queries**: `Match`, `MultiMatch`, `QueryString`, etc.
//! - **Compound queries**: `Bool`, `Boosting`, `ConstantScore`, etc.
//!
//! # Examples
//!
//! See the individual module documentation and test files for more examples.
//!
//! # Crate Organization
//!
//! - [`model`] - Core query structure and builder
//! - [`term`] - Term-level queries
//! - [`full_text`] - Full-text search queries
//! - [`compound_query`] - Compound query combinators
//! - [`misc`] - Supporting types and utilities

pub mod compound_query;
pub mod full_text;
pub mod misc;
pub mod term;

/// Core query model and builder types
pub mod model {
    use crate::misc::query_field::QueryField;
    use serde::Serialize;
    use serde_json::Value;

    /// Examples
    /// ```
    /// use os_query_builder_rs::full_text::multi_match::MultiMatch;
    /// use os_query_builder_rs::misc::operator::Operator;
    /// use os_query_builder_rs::misc::query_field::QueryField;
    /// use os_query_builder_rs::misc::r#type::Type;
    /// use os_query_builder_rs::model::Query;
    ///
    /// let multi_match = MultiMatch::new()
    ///             .fields(vec!["brands", "articles"])
    ///             .value("oc47")
    ///             .operator(Operator::And)
    ///             .query_type(Type::BestFields)
    ///             .boost(2)
    ///             .minimum_should_match("90%");
    ///
    /// let query = Query::new()
    ///             .source(vec!["test"])
    ///             .query(multi_match);
    /// ```
    #[derive(Debug, Default, Clone, Serialize)]
    pub struct Query {
        #[serde(skip_serializing_if = "Option::is_none")]
        from: Option<usize>,

        #[serde(skip_serializing_if = "Option::is_none")]
        size: Option<usize>,

        #[serde(rename = "_source", skip_serializing_if = "Option::is_none")]
        source: Option<Vec<String>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        query: Option<QueryField>,

        #[serde(skip_serializing_if = "Option::is_none")]
        aggs: Option<Value>,
    }

    impl Query {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn source<F, T>(self, source: F) -> Self
        where
            F: IntoIterator<Item = T>,
            T: Into<String>,
        {
            Self {
                source: Some(source.into_iter().map(|x| x.into()).collect()),
                ..self
            }
        }

        pub fn query<T: Into<QueryField> + Serialize>(self, query: T) -> Self {
            Self {
                query: Some(query.into()),
                ..self
            }
        }

        pub fn from<T: Into<usize> + Serialize>(self, from: T) -> Self {
            Self {
                from: Some(from.into()),
                ..self
            }
        }

        pub fn size<T: Into<usize> + Serialize>(self, size: T) -> Self {
            Self {
                size: Some(size.into()),
                ..self
            }
        }

        pub fn aggs<T: Into<Value> + Serialize>(self, aggs: T) -> Self {
            Self {
                aggs: Some(aggs.into()),
                ..self
            }
        }
    }
}

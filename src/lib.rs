pub mod full_text;
pub mod misc;

pub mod model {
    use serde::Serialize;
    use serde_json::Value;
    use crate::misc::query_field::QueryField;


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
    ///            .source(vec!["test"])
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
                F: IntoIterator<Item=T>,
                T: Into<String>
        {
            Self {
                source: Some(source
                    .into_iter()
                    .map(|x| x.into())
                    .collect()),
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
                from: Some(size.into()),
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

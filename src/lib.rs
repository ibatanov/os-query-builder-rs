pub mod query;
pub mod query_field;
pub mod misc;

pub mod model {
    use serde::Serialize;
    use crate::query::aggs::Aggregates;

    use crate::query_field::QueryField;

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
        aggs: Option<Aggregates>,
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

        pub fn aggs<T: Into<Aggregates> + Serialize>(self, aggs: T) -> Self {
            Self {
                aggs: Some(aggs.into()),
                ..self
            }
        }
    }
}

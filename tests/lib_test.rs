#[cfg(test)]
mod tests {
    use serde_json::json;

    use os_query_builder_rs::misc::operator::Operator;
    use os_query_builder_rs::model::Query;
    use os_query_builder_rs::query::multi_match::MultiMatchQuery;
    use os_query_builder_rs::query::query_field::QueryField;
    use os_query_builder_rs::query::r#match::MatchQuery;

    #[test]
    fn query_test() {
        let match_test = MatchQuery::new()
            .field("brand")
            .value("bosh");

        let query = Query::new()
            .source(vec!["test"])
            .query(QueryField::Match(match_test));

        let json_str = json!({"query":{"match":{"brand":{"query":"bosh"}}},"_source":["test"]});
        let json = serde_json::to_value(query).unwrap();
        assert_eq!(json_str, json);
    }

    #[test]
    fn match_test() {
        let mq = MatchQuery::new()
            .field("brands")
            .value("bosch")
            .boost(2f64)
            .operator(Operator::Or);

        let json_str = json!({"brands":{"boost":2.0,"operator":"or","query":"bosch"}});
        let json = serde_json::to_value(mq).unwrap();
        assert_eq!(json_str, json);
    }

    #[test]
    fn multi_match_test() {
        let mm = MultiMatchQuery::new()
            .fields(vec!["brands", "articles"])
            .value("oc47");

        let json_str = json!({"fields":["brands","articles"],"query":"oc47"});
        let json = serde_json::to_value(mm).unwrap();
        assert_eq!(json_str, json);
    }
}

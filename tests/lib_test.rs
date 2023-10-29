#[cfg(test)]
mod tests {
    use serde_json::json;

    use os_query_builder_rs::full_text::multi_match::MultiMatchQuery;
    use os_query_builder_rs::full_text::r#match::MatchQuery;
    use os_query_builder_rs::misc::operator::Operator;
    use os_query_builder_rs::misc::query_field::QueryField;
    use os_query_builder_rs::misc::r#type::Type;
    use os_query_builder_rs::model::Query;

    #[test]
    fn query_test() {
        let match_test = MatchQuery::new()
            .field("brand")
            .value("My Brand");

        let query = Query::new()
            .source(vec!["test"])
            .query(QueryField::Match(match_test));

        let json_str = json!({"query":{"match":{"brand":{"query":"My Brand"}}},"_source":["test"]});
        let json = serde_json::to_value(query).unwrap();
        assert_eq!(json_str, json);
    }

    #[test]
    fn match_test() {
        let mq = MatchQuery::new()
            .field("brands")
            .value("My Brand")
            .boost(2f64)
            .operator(Operator::Or);

        let json_str = json!({"brands":{"boost":2.0,"operator":"or","query":"My Brand"}});
        let json = serde_json::to_value(mq).unwrap();
        assert_eq!(json_str, json);
    }

    #[test]
    fn multi_match_test() {
        let multi_match = MultiMatchQuery::new()
            .fields(vec!["brands", "articles"])
            .value("oc47")
            .operator(Operator::And)
            .search_type(Type::BestFields)
            .boost(2)
            .minimum_should_match(2u64);
        let json_str = json!({"boost":2.0,"fields":["brands","articles"],"minimum_should_match":2,"operator":"and","query":"oc47","type":"best_fields"});
        let json = serde_json::to_value(multi_match).unwrap();
        assert_eq!(json_str, json);
    }
}

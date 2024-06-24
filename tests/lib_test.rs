#[cfg(test)]
mod tests {
    use serde_json::json;

    use os_query_builder_rs::full_text::multi_match::MultiMatch;
    use os_query_builder_rs::full_text::terms::{Terms, TermsLookup};
    use os_query_builder_rs::full_text::r#match::Match;
    use os_query_builder_rs::misc::operator::Operator;
    use os_query_builder_rs::misc::r#type::Type;
    use os_query_builder_rs::model::Query;

    #[test]
    fn query_test() {
        let match_test = Match::new()
            .field("brand")
            .value("My Brand");

        let query = Query::new()
            .source(vec!["test"])
            .query(match_test);

        let json_str = json!({"query":{"match":{"brand":{"query":"My Brand"}}},"_source":["test"]});
        let json = serde_json::to_value(query).unwrap();
        assert_eq!(json_str, json);
    }

    #[test]
    fn match_test() {
        let mq = Match::new()
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
        let multi_match = MultiMatch::new()
            .fields(vec!["brands", "articles"])
            .value("oc47")
            .operator(Operator::And)
            .query_type(Type::BestFields)
            .boost(2)
            .minimum_should_match("90%");
        let json_str = json!({"boost":2.0,"fields":["brands","articles"],"minimum_should_match":"90%","operator":"and","query":"oc47","type":"best_fields"});
        let json = serde_json::to_value(multi_match).unwrap();
        assert_eq!(json_str, json);
    }



    #[test]
    fn query_terms_query_test() {

        let terms = Terms::new_with_terms_query("brand_id", vec!["266", "267", "268"]);
        let query = Query::new().query(terms);
        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "query": {
                "terms": {
                    "brand_id": ["266", "267", "268"]
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }

    #[test]
    fn query_terms_query_with_boost_test() {

        let terms = Terms::new_with_terms_query("brand_id", vec!["266", "267", "268"]).boost(0.9);
        let query = Query::new().query(terms);
        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "query": {
                "terms": {
                    "brand_id": ["266", "267", "268"],
                    "boost": 0.9
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }

    #[test]
    fn query_terms_query_with_boost_test_1() {

        let terms = Terms::new_with_terms_query("product_id", vec!["128660147","127875288",]).boost(0.7);
        let query = Query::new().query(terms)
            .source(vec!["product_id", "brand", "article", "name",]);

        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "_source": [
                "product_id",
                "brand",
                "article",
                "name",
            ],
            "query": {
                "terms": {
                    "product_id": [
                        "128660147",
                        "127875288"
                    ],
                    "boost": 0.7
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }



    #[test]
    fn query_terms_lookup_test() {

        let terms_lookup = TermsLookup::new("sku", "131356111", "brand_id");
        let terms = Terms::new_with_terms_lookup("brand_id", terms_lookup);
        let query = Query::new().query(terms);
        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "query": {
                "terms": {
                    "brand_id": {
                        "index": "sku",
                        "id": "131356111",
                        "path": "brand_id"
                    }
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }

    #[test]
    fn query_terms_lookup_with_boost_test() {

        let boost = 0.7;
        let terms_lookup = TermsLookup::new("sku", "131356111", "brand_id");
        let terms = Terms::new_with_terms_lookup("brand_id", terms_lookup).boost(boost);
        let query = Query::new().query(terms);
        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "query": {
                "terms": {
                    "brand_id": {
                        "index": "sku",
                        "id": "131356111",
                        "path": "brand_id"
                    },
                    "boost": boost
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }

    #[test]
    fn query_terms_lookup_with_routing_test() {

        let boost = 0.7;
        let terms_lookup = TermsLookup::new("sku", "131356111", "brand_id").routing("brand_id");
        let terms = Terms::new_with_terms_lookup("brand_id", terms_lookup).boost(boost);
        let query = Query::new().query(terms);
        let json_actual = serde_json::to_value(query).unwrap();

        let json_expected = json!({
            "query": {
                "terms": {
                    "brand_id": {
                        "index": "sku",
                        "id": "131356111",
                        "path": "brand_id",
                        "routing": "brand_id",
                    },
                    "boost": boost
                }
            }
        });

        assert_eq!(json_expected, json_actual);
    }


    #[test]
    fn term_query_test() {
        let terms_query = Terms::new_with_terms_query("product_id", vec!["1", "2", "3"]);
        let json_actual = serde_json::to_value(terms_query).unwrap();

        let json_expected = json!({"product_id": ["1", "2", "3"]});

        assert_eq!(json_expected, json_actual);
    }
    #[test]
    fn term_query_with_boost_test() {

        let boost = 0.9;
        let terms = Terms::new_with_terms_query("product_id", vec!["1", "2", "3"]).boost(boost);
        let json_actual = serde_json::to_value(terms).unwrap();

        let json_expected = json!({"product_id": ["1", "2", "3"], "boost": boost});


        assert_eq!(json_expected, json_actual);
    }


    #[test]
    fn terms_lookup_test() {

        let terms_lookup = Terms::new_with_terms_lookup("product_id",
                                             TermsLookup::new("classes",
                                                              "102",
                                                              "enrolled_students.id_list"
                                             )
        );
        let json_actual = serde_json::to_value(terms_lookup).unwrap();

        let json_expected = json!({
            "product_id": {
                "index": "classes",
                "id": "102",
                "path": "enrolled_students.id_list"
            }
        });

        assert_eq!(json_expected, json_actual);
    }
    #[test]
    fn terms_lookup_with_boost_test() {

        let boost = 0.65;
        let terms_lookup = Terms::new_with_terms_lookup("product_id",
                                                        TermsLookup::new("classes",
                                                                         "102",
                                                                         "enrolled_students.id_list"
                                                        ).routing("brand_id")
        ).boost(boost);
        let json_actual = serde_json::to_value(terms_lookup).unwrap();

        let json_expected = json!({
            "product_id": {
                "index": "classes",
                "id": "102",
                "path": "enrolled_students.id_list",
                "routing": "brand_id"
            },
            "boost": boost
        });

        assert_eq!(json_expected, json_actual);
    }
    #[test]
    fn terms_lookup_with_routing_test() {

        let terms_lookup = Terms::new_with_terms_lookup("product_id",
                                                        TermsLookup::new("classes",
                                                                         "102",
                                                                         "enrolled_students.id_list"
                                                        ).routing("brand_id")
        );
        let json_actual = serde_json::to_value(terms_lookup).unwrap();

        let json_expected = json!({
            "product_id": {
                "index": "classes",
                "id": "102",
                "path": "enrolled_students.id_list",
                "routing": "brand_id"
            }
        });

        assert_eq!(json_expected, json_actual);
    }
    #[test]
    fn terms_lookup_with_routing_and_boost_test() {

        let boost = 0.65;
        let terms_lookup = Terms::new_with_terms_lookup("product_id",
                                                        TermsLookup::new("classes",
                                                                         "102",
                                                                         "enrolled_students.id_list"
                                                        ).routing("brand_id")
        ).boost(boost);
        let json_actual = serde_json::to_value(terms_lookup).unwrap();

        let json_expected = json!({
            "product_id": {
                "index": "classes",
                "id": "102",
                "path": "enrolled_students.id_list",
                "routing": "brand_id"
            },
            "boost": boost
        });

        assert_eq!(json_expected, json_actual);
    }
}

use os_query_builder_rs::{model::Query, term::term::Term, term::terms::{Terms, TermsLookup}};
use serde_json::json;

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
    let terms = Terms::new_with_terms_query("product_id", vec!["128660147", "127875288"]).boost(0.7);
    let query = Query::new()
        .query(terms)
        .source(vec!["product_id", "brand", "article", "name"]);

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
    let terms_lookup = TermsLookup::new("sku", "131356111","brand_id");
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
fn term_query_test_with_different_types_str_and_string() {
    let terms_query = Terms::new_with_terms_query(
        "product_id",
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );
    let json_actual = serde_json::to_value(terms_query).unwrap();

    let json_expected = json!({"product_id": ["1", "2", "3"]});

    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_query_test_with_different_types_str_and_chars() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!['1', '2', '3']);
    let json_actual = serde_json::to_value(terms_query).unwrap();

    let json_expected = json!({"product_id": ["1", "2", "3"]});

    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_query_with_boost_test() {
    let boost = 0.9;
    let terms = Terms::new_with_terms_query("product_id", vec![1, 2, 3]).boost(boost);
    let json_actual = serde_json::to_value(terms).unwrap();

    let json_expected = json!({"product_id": [1, 2, 3], "boost": boost});

    assert_eq!(json_expected, json_actual);
}

#[test]
fn terms_lookup_test() {
    let terms_lookup = Terms::new_with_terms_lookup(
        "product_id",
        TermsLookup::new("classes", "102", "enrolled_students.id_list")
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
    let terms_lookup = Terms::new_with_terms_lookup(
        "product_id",
        TermsLookup::new("classes","102","enrolled_students.id_list").routing("brand_id"),
    )
    .boost(boost);
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
    let terms_lookup = Terms::new_with_terms_lookup(
        "product_id",
        TermsLookup::new("classes", "102", "enrolled_students.id_list")
            .routing("brand_id"),
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
    let terms_lookup = Terms::new_with_terms_lookup(
        "product_id",
        TermsLookup::new("classes","102","enrolled_students.id_list").routing("brand_id"),
    )
    .boost(boost);
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
fn term_test() {
    let term = Term::new("brand_name", "FIAT");
    let json_actual = json!(term);
    
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_test_with_value_is_int() {
    let term = Term::new("brand_id", 1 as i64);
    let json_actual = json!(term);
    
    let json_expected = json!({
        "brand_id": {
            "value": 1,
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_test_with_value_is_float64() {
    let term = Term::new("brand_id", 1.3211);
    let json_actual = json!(term);
    
    let json_expected = json!({
        "brand_id": {
            "value": 1.3211,
        }
    });
    assert_eq!(json_expected, json_actual);
}


#[test]
fn term_test_with_value_is_float32() { 
    
    let value = 24334.1232131 as f32;
    let term = Term::new("brand_id", value);
    let json_actual = json!(term);
    
    let json_expected = json!({
        "brand_id": {
            "value": value,
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_with_boost_test() {
    let term = Term::new("brand_name", "FIAT").boost(0.65);
    
    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "boost": 0.65
        }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_with_case_insensitive_test() {
    let term = Term::new("brand_name", "FIAT").case_insensitive(true);
    
    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "case_insensitive": true
        }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_with_boost_and_case_insensitive_test() {
    let term = Term::new("brand_name", "FIAT")
        .case_insensitive(true)
        .boost(0.6);
    
    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "case_insensitive": true,
            "boost": 0.6
        }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn query_term_with_boost_and_case_insensitive_test() {
    let term = Term::new("article", "43935055")
        .case_insensitive(true)
        .boost(0.6);
    
    let query = Query::new().query(term);
    
    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
            "term": {
                "article": {
                    "boost":0.6,
                    "case_insensitive":true,
                    "value":"43935055"
                }
            }
        }
    });
    
    assert_eq!(json_expected, json_actual);
}

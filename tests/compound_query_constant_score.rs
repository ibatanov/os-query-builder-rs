use serde_json::json;
use os_query_builder_rs::compound_query::bool::Bool;
use os_query_builder_rs::compound_query::boosting::Boosting;
use os_query_builder_rs::compound_query::constant_score::ConstantScore;
use os_query_builder_rs::full_text::r#match::Match;
use os_query_builder_rs::misc::query_field::QueryField;
use os_query_builder_rs::model::Query;

#[test]
fn example_from_docs_test() {
    let match_value = Match::new().field("text_entry").value("Hamlet");
    let constant_score = ConstantScore::new(match_value).boost(1.2);
    let query = Query::new().query(constant_score);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "constant_score": {
                "filter": {
                    "match": {
                        "text_entry": {
                            "query":"Hamlet"
                        }
                    }
                },
                "boost": 1.2
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn simple_match_test() {
    let match_value = Match::new().field("brand").value("Инструменты СТО");
    let constant_score = ConstantScore::new(match_value);
    let query = Query::new().query(constant_score);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "constant_score": {
                "filter": {
                    "match": {
                        "brand": {
                            "query":"Инструменты СТО"
                        }
                    }
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn with_nested_bool_test() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![QueryField::Match(match_value)]);
    let constant_score = ConstantScore::new(boolean);
    let query = Query::new().query(constant_score);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "constant_score": {
                "filter": {
                    "bool": {
                        "must": [
                            {
                                "match": {
                                    "brand": {
                                        "query": "FIAT"
                                    }
                                }
                            }
                        ]
                    }
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_nested_boosting_with_boost_test() {
    let match_positive = Match::new()
        .field("product_id")
        .value("46118216");
    let match_negative = Match::new()
        .field("name")
        .value("Деталь");
    let boosting = Boosting::new(match_positive, match_negative)
        .negative_boost(0.1f64);

    let constant_score = ConstantScore::new(boosting);
    let query = Query::new().query(constant_score);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "constant_score": {
                "filter": {
                   "boosting": {
                        "positive": {
                            "match": {
                                "product_id": {
                                    "query": "46118216"
                                }
                            }
                        },
                        "negative": {
                            "match": {
                                "name": {
                                    "query": "Деталь"
                                }
                            }
                        },
                        "negative_boost": 0.1
                    }
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}
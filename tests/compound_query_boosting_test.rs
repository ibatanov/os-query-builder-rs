use serde_json::json;
use os_query_builder_rs::compound_query::bool::Bool;
use os_query_builder_rs::compound_query::boosting::Boosting;
use os_query_builder_rs::full_text::r#match::Match;
use os_query_builder_rs::model::Query;


#[test]
fn example_docs_test() {

    let match_positive = Match::new()
        .field("article_name")
        .value("pitcher");

    let match_negative = Match::new()
        .field("article_name")
        .value("glass crystal water");

    let boosting = Boosting::new(match_positive, match_negative)
        .negative_boost(0.1f64);

    let query = Query::new().query(boosting);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "boosting": {
                "positive": {
                    "match": {
                        "article_name": {
                            "query": "pitcher"
                        }
                    }
                },
                "negative": {
                    "match": {
                        "article_name": {
                            "query": "glass crystal water"
                        }
                    }
                },
                "negative_boost": 0.1
            }
        }
    });
    assert_eq!(actual_json, excepted_json);
}

#[test]
fn simple_test() {

    let match_positive = Match::new()
        .field("product_id")
        .value("46118216");

    let match_negative = Match::new()
        .field("name")
        .value("Деталь");

    let boosting = Boosting::new(match_positive, match_negative)
        .negative_boost(0.1f64);

    let query = Query::new().query(boosting);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
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
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_nested_boolean_query() {

    let match_value = Match::new().field("brand").value("FIAT");
    let boolean_positive = Bool::new().must(vec![match_value]);

    let match_negative = Match::new()
        .field("name")
        .value("Деталь");

    let boosting = Boosting::new(boolean_positive, match_negative)
        .negative_boost(0.1f64);

    let query = Query::new().query(boosting);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "boosting": {
                "positive": {
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
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn with_nested_with_negative_boolean_query_() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean_negative = Bool::new().must(vec![match_value]);

    let match_positive = Match::new()
        .field("name")
        .value("Деталь");

    let boosting = Boosting::new(match_positive, boolean_negative)
        .negative_boost(0.1f64);

    let query = Query::new().query(boosting);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "boosting": {
                "positive": {
                    "match": {
                        "name": {
                            "query": "Деталь"
                        }
                    }
                },
                "negative": {
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
                },
                "negative_boost": 0.1
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}
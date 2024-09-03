use serde_json::json;
use os_query_builder_rs::compound_query::bool::Bool;
use os_query_builder_rs::compound_query::boosting::Boosting;
use os_query_builder_rs::compound_query::constant_score::ConstantScore;
use os_query_builder_rs::compound_query::disjunction_max::DisMax;
use os_query_builder_rs::full_text::r#match::Match;
use os_query_builder_rs::misc::query_field::QueryField;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::term::Term;

#[test]
fn example_docs_test() {
    let match_positive = Match::new()
        .field("title")
        .value("Shakespeare poems");
    let match_negative = Match::new()
        .field("body")
        .value("Shakespeare poems");
    let boosting = DisMax::new(vec![match_positive, match_negative]);

    let query = Query::new().query(boosting);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "dis_max": {
                "queries": [
                    { "match": { "title": { "query": "Shakespeare poems" }}},
                    { "match": { "body":  { "query": "Shakespeare poems" }}}
                ]
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_simple_queries() {
    let match_positive = Match::new()
        .field("name")
        .value("Автозапчасть");
    let match_negative = Match::new()
        .field("product_id")
        .value("46118319");
    let term = Term::new("catalogue_id", 13558);
    let dis_max = DisMax::new(vec![QueryField::Match(match_positive), QueryField::Match(match_negative), QueryField::Term(term)]);
    let query = Query::new().query(dis_max);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "dis_max": {
                "queries": [
                    { "match": { "name": { "query": "Автозапчасть" }}},
                    { "match": { "product_id":  { "query": "46118319" }}},
                    { "term": {"catalogue_id": { "value": 13558 }}}
                ]
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn with_compound_queries() {
    let match_value_1 = Match::new().field("name").value("Деталь");
    let match_value_2 = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![match_value_1, match_value_2]);

    let match_value = Match::new().field("brand").value("FIAT");
    let boolean_negative = Bool::new().must(vec![match_value]);
    let match_positive = Match::new()
        .field("name")
        .value("Деталь");
    let boosting = Boosting::new(match_positive, boolean_negative)
        .negative_boost(0.1f64);

    let match_value = Match::new().field("brand").value("Инструменты СТО");
    let constant_score = ConstantScore::new(match_value);

    let match_positive = Match::new()
        .field("name")
        .value("Автозапчасть");
    let match_negative = Match::new()
        .field("product_id")
        .value("46118319");
    let term = Term::new("catalogue_id", 13558);
    let simple_dis_max = DisMax::new(vec![QueryField::Match(match_positive), QueryField::Match(match_negative), QueryField::Term(term)]);

    let dis_max = DisMax::new(vec![
        QueryField::Bool(boolean),
        QueryField::Boosting(boosting),
        QueryField::ConstantScore(constant_score),
        QueryField::DisMax(simple_dis_max)
    ]);
    let query = Query::new().query(dis_max);
    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "dis_max": {
                "queries": [
                    {
                        "bool": {
                            "must": [
                            {
                                "match": {
                                    "name": {
                                        "query": "Деталь"
                                    }
                                }
                            },
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
                    {
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
                    },
                    {
                        "constant_score": {
                            "filter": {
                                "match": {
                                    "brand": {
                                        "query":"Инструменты СТО"
                                    }
                                }
                            }
                        }
                    },
                    {
                        "dis_max": {
                            "queries": [
                                { "match": { "name": { "query": "Автозапчасть" }}},
                                { "match": { "product_id":  { "query": "46118319" }}},
                                { "term": {"catalogue_id": { "value": 13558 }}}
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}
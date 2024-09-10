use serde_json::json;
use os_query_builder_rs::misc::relation::Relation;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::range::Range;

#[test]
fn simple_example_from_docs_test() {
    let range = Range::new()
        .field("line_id")
        .gte(10)
        .lte(20);
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "line_id": {
                    "gte": 10,
                    "lte": 20
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_date_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("2019/01/01")
        .lte("2019/12/31");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "gte": "2019/01/01",
                    "lte": "2019/12/31"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}



#[test]
fn with_format_date_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("01/01/2022")
        .lte("31/12/2022")
        .format("dd/MM/yyyy");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "gte": "01/01/2022",
                    "lte": "31/12/2022",
                     "format": "dd/MM/yyyy"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}



#[test]
fn with_missing_date_components_date_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("2022")
        .lte("2022-12-31");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "gte": "2022",
                    "lte": "2022-12-31"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn first_with_relative_dates_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("2019/01/01||-1y-1d");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "gte": "2019/01/01||-1y-1d"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn second_with_relative_dates_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("now-1y/M");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "gte": "now-1y/M"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_time_zone_example_from_docs_test() {
    let range = Range::new()
        .field("created")
        .gte("2022-04-17T06:00:00")
        .time_zone("-04:00");
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "range": {
                "created": {
                    "time_zone": "-04:00",
                    "gte": "2022-04-17T06:00:00"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn all_fields_for_range_int_values_test() {
    let range = Range::new()
        .field("product_id")
        .gt(46118215)
        .gte(46118216)
        .lt(46118456)
        .lte(46118455)
        .boost(10)
        .relation(Relation::Intersects);
    let query = Query::new().query(range);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
           "range": {
                "product_id": {
                    "gt": 46118215,
                    "gte": 46118216,
                    "lt": 46118456,
                    "lte": 46118455,
                    "boost": 10.0,
                    "relation": "INTERSECTS"
                }
            }
        }
    });

    assert_eq!(actual_json, excepted_json);
}
use os_query_builder_rs::full_text::query_string::QueryString;
use serde_json::json;
use os_query_builder_rs::misc::fuzziness::Fuzziness;
use os_query_builder_rs::model::Query;

#[test]
fn first_example_docs_test() {

    let query_sting = QueryString::new().query("the wind AND (rises OR rising)");
    let query = Query::new()
        .query(query_sting);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "query_string": {
                "query": "the wind AND (rises OR rising)"
            }
        }
    });

    assert_eq!(excepted_json, actual_json);
}


#[test]
fn first_example_from_docs_with_fuzziness_auto_test() {
    let query_sting = QueryString::new()
        .query("the wind AND (rises OR rising)")
        .fuzziness(Fuzziness::Auto);
    let query = Query::new()
        .query(query_sting);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "query_string": {
                "query": "the wind AND (rises OR rising)",
                "fuzziness": "AUTO"
            }
        }
    });

    assert_eq!(excepted_json, actual_json);
}


#[test]
fn first_example_from_docs_with_fuzziness_uint_test() {

    let query_sting = QueryString::new()
        .query("the wind AND (rises OR rising)")
        .fuzziness(Fuzziness::Uint(10));
    let query = Query::new()
        .query(query_sting);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "query_string": {
                "query": "the wind AND (rises OR rising)",
                "fuzziness": 10
            }
        }
    });

    assert_eq!(excepted_json, actual_json);
}

#[test]
fn simple_test() {

    let query_sting = QueryString::new()
        .query("historical epic heroic")
        .minimum_should_match("2");
    let query = Query::new()
        .query(query_sting);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "query_string": {
              "query": "historical epic heroic",
              "minimum_should_match": "2"
            }
        }
    });

    assert_eq!(excepted_json, actual_json);
}
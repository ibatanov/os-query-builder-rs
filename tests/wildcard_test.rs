use serde_json::json;
use os_query_builder_rs::misc::rewrite::Rewrite;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::wildcard::Wildcard;

#[test]
fn example_from_docs() {
    let wildcard = Wildcard::new()
        .field("speaker")
        .value("H*Y")
        .case_insensitive(false);
    let query = Query::new().query(wildcard);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "wildcard": {
          "speaker": {
            "value": "H*Y",
            "case_insensitive": false
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn example_from_docs_with_all_fields() {
    let wildcard = Wildcard::new()
        .field("speaker")
        .value("H*Y")
        .case_insensitive(false)
        .boost(1.5f64)
        .rewrite(Rewrite::ScoringBoolean);
    let query = Query::new().query(wildcard);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "wildcard": {
          "speaker": {
            "value": "H*Y",
            "case_insensitive": false,
            "boost": 1.5,
            "rewrite": "scoring_boolean"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn test_with_all_fields() {
    let wildcard = Wildcard::new()
        .field("name")
        .value("Автозапчасть")
        .case_insensitive(true)
        .boost(2f64)
        .rewrite(Rewrite::TopTerms(100));
    let query = Query::new().query(wildcard);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "wildcard": {
          "name": {
            "value": "Автозапчасть",
            "case_insensitive": true,
            "boost": 2.0,
            "rewrite": "top_terms_100"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn test_with_main_field() {
    let wildcard = Wildcard::new()
        .field("name")
        .value("Автозапчасть");
    let query = Query::new().query(wildcard);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "wildcard": {
          "name": {
            "value": "Автозапчасть"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
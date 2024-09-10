use serde_json::json;
use os_query_builder_rs::compound_query::bool::Bool;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::exists::Exists;

#[test]
fn example_from_docs_test() {
    let exists = Exists::new().field("description");
    let query = Query::new().query(exists);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "exists": {
          "field": "description"
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn example_from_docs_with_bool_compound_query_test() {

    let exists = Exists::new().field("description");
    let bool_query = Bool::new()
        .must_not(vec![exists]);

    let query = Query::new().query(bool_query);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "bool": {
          "must_not": [
            {
              "exists": {
                "field": "description"
              }
            }
         ]
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn test_index_test() {
    let exists = Exists::new().field("catalogue_id");
    let bool_query = Bool::new()
        .must(vec![exists]);

    let query = Query::new().query(bool_query);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "bool": {
          "must": [
            {
              "exists": {
                "field": "catalogue_id"
              }
            }
         ]
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
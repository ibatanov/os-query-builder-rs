use serde_json::json;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::ids::IDs;

#[test]
fn with_int_values_test() {
    let ids = IDs::new().values(vec![45664230, 45664262]);
    let query = Query::new().query(ids);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "ids": {
          "values": [
            45664230,
            45664262
          ]
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_string_values_test() {
    let ids = IDs::new().values(vec!["45664230", "45664262"]);
    let query = Query::new().query(ids);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "ids": {
          "values": [
            "45664230",
            "45664262"
          ]
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_boost_test() {
    let ids = IDs::new()
        .values(vec!["45664230", "45664262"])
        .boost(10.0);
    let query = Query::new().query(ids);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "ids": {
          "values": [
            "45664230",
            "45664262"
          ],
          "boost": 10.0
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
use serde_json::json;
use os_query_builder_rs::misc::rewrite::Rewrite;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::prefix::Prefix;

#[test]
fn prefix_example_from_docs() {
    let prefix = Prefix::new()
        .field("speaker")
        .value("KING H");
    let query = Query::new().query(prefix);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "prefix": {
          "speaker": {
            "value": "KING H"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn prefix_with_main_field() {
    let prefix = Prefix::new()
        .field("catalogue_name")
        .value("Крыло заднее")
        .boost(2)
        .case_insensitive(true)
        .rewrite(Rewrite::ConstantScoreBoolean);
    let query = Query::new().query(prefix);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "prefix": {
          "catalogue_name": {
            "value": "Крыло заднее",
            "boost": 2.0,
            "case_insensitive": true,
            "rewrite": "constant_score_boolean"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
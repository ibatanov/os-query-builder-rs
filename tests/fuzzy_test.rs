use serde_json::json;
use os_query_builder_rs::{
    misc::{
        fuzziness::Fuzziness,
        rewrite::Rewrite
    },
    model::Query,
    term::fuzzy::Fuzzy
};

#[test]
fn example_first_from_docs_test() {

    let fuzzy = Fuzzy::new()
        .field("speaker")
        .value("HALET");
    let query = Query::new().query(fuzzy);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "fuzzy": {
          "speaker": {
            "value": "HALET"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn example_second_from_docs_test() {

    let fuzzy = Fuzzy::new()
        .field("speaker")
        .value("HALET")
        .fuzziness(Fuzziness::Uint(2))
        .max_expansions(40u64)
        .prefix_length(0u64)
        .transpositions(true)
        .rewrite(Rewrite::ConstantScore);
    let query = Query::new().query(fuzzy);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "fuzzy": {
          "speaker": {
            "value": "HALET",
            "fuzziness": 2,
            "max_expansions": 40,
            "prefix_length": 0,
            "transpositions": true,
            "rewrite": "constant_score"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
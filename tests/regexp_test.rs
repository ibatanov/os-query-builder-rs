use serde_json::json;
use os_query_builder_rs::misc::regexp_flag::RegexpFlag;
use os_query_builder_rs::misc::rewrite::Rewrite;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::regexp::Regexp;

#[test]
fn example_from_docs() {
    let regexp = Regexp::new()
        .field("play_name")
        .value("[a-zA-Z]amlet");
    let query = Query::new().query(regexp);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "regexp": {
          "play_name": {
            "value" :"[a-zA-Z]amlet"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn with_main_fields() {
    let regexp = Regexp::new()
        .field("name")
        .value("А[а-я]+");
    let query = Query::new()
        .query(regexp)
        .from(10u8)
        .size(20u16);

    let actual_json = json!(query);
    let excepted_json = json!({
      "size":20,
      "from": 10,
      "query": {
        "regexp": {
          "name": {
            "value": "А[а-я]+"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn regexp_with_all_fields() {
    let regexp = Regexp::new()
        .field("name")
        .rewrite(Rewrite::TopTerms(100))
        .boost(100)
        .flags(RegexpFlag::AnyString)
        .max_determinized_states(10000u64)
        .case_insensitive(true)
        .value("k.*y");
    let query = Query::new()
        .query(regexp)
        .from(10u8)
        .size(20u16);

    let actual_json = json!(query);
    let excepted_json = json!({
      "size":20,
      "from": 10,
      "query": {
        "regexp": {
           "name": {
             "value": "k.*y",
             "flags": "ANYSTRING",
             "case_insensitive": true,
             "max_determinized_states": 10000,
             "rewrite": "top_terms_100",
             "boost": 100.0
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}
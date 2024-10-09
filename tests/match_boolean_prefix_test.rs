use serde_json::json;
use os_query_builder_rs::full_text::match_boolean_prefix::MatchBoolPrefix;
use os_query_builder_rs::misc::{
    fuzziness::Fuzziness,
    rewrite::Rewrite,
    operator::Operator
};
use os_query_builder_rs::model::Query;

#[test]
fn example_from_doc_test() {
    let value = MatchBoolPrefix::new()
        .field("title")
        .query("rise the wi")
        .analyzer("stop");
    let query = Query::new().query(value);
    let actual = json!(query);
    let excepted = json!({
      "query": {
        "match_bool_prefix": {
          "title": {
            "query": "rise the wi",
            "analyzer": "stop"
          }
        }
     }
    });


    assert_eq!(actual, excepted);
}


#[test]
fn match_bool_prefix_with_fuzzy_rewrite() {
    let value = MatchBoolPrefix::new()
        .field("name")
        .query("Автозапчасть")
        .fuzzy_rewrite(Rewrite::TopTerms(1))
        .analyzer("stop");
    let query = Query::new().query(value);
    let actual = json!(query);
    let excepted = json!({
      "query": {
        "match_bool_prefix": {
          "name": {
            "query": "Автозапчасть",
            "analyzer": "stop",
            "fuzzy_rewrite": "top_terms_1"
          }
        }
    }
    });

    assert_eq!(actual, excepted);
}


#[test]
fn match_bool_prefix_with_all_fields() {
    let value = MatchBoolPrefix::new()
        .field("name")
        .query("Автозапчасть")
        .operator(Operator::And)
        .fuzziness(Fuzziness::Auto)
        .minimum_should_match("90%")
        .prefix_length(200u64)
        .max_expansions(50u64)
        .fuzzy_rewrite(Rewrite::ConstantScore)
        .fuzzy_transpositions(false)
        .analyzer("stop");
    let query = Query::new().query(value);
    let actual = json!(query);
    let excepted = json!({
      "query": {
        "match_bool_prefix": {
          "name": {
            "query": "Автозапчасть",
            "analyzer": "stop",
            "fuzziness": "AUTO",
            "fuzzy_rewrite": "constant_score",
            "operator": "and",
            "max_expansions": 50,
            "prefix_length": 200,
            "fuzzy_transpositions": false,
            "minimum_should_match": "90%",
          }
        }
    }
    });

    assert_eq!(actual, excepted);
}
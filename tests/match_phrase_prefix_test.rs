use os_query_builder_rs::{full_text::match_phrase_prefix::MatchPhrasePrefix, model::Query};
use serde_json::json;

#[test]
fn full_match_phrase_prefix_test() {
    let match_phrase_prefix = MatchPhrasePrefix::new().field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    let json = json!(match_phrase_prefix);
    let json_str = json!({
        "brand": {
            "query": "DPH",
            "analyzer": "stop",
            "slop": 3,
            "max_expansions": 60
        }
    });

    assert_eq!(json_str, json);
}

#[test]
fn query_match_phrase_prefix_test() {
    let match_phrase_test_test = MatchPhrasePrefix::new().field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    
    let query = Query::new()
        .source(vec!["brand"])
        .query(match_phrase_test_test);

    let json = json!(query);
    let json_str = json!({
        "query":{
            "match_phrase_prefix":{
                "brand":{
                    "query": "DPH",
                    "analyzer": "stop",
                    "slop": 3,
                    "max_expansions": 60
                }
            }
        },
        "_source":["brand"]
    });
    println!("{json}");
    assert_eq!(json_str, json);
}
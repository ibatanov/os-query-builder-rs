use serde_json::json;

use os_query_builder_rs::full_text::match_phrase::MatchPhrase;
use os_query_builder_rs::model::Query;

#[test]
fn match_phrase_test() {
    let match_phrase = MatchPhrase::new()
        .field("brand")
        .value("DPH")
        .analyzer("english")
        .slop(2u64)
        .zero_terms_query("all");

    let json = json!(match_phrase);
    let json_str = json!({
        "brand":{
            "query": "DPH",
            "analyzer": "english",
            "slop": 2,
            "zero_terms_query": "all"
        }
    });
    
    assert_eq!(json_str, json);
}


#[test]
fn query_match_phrase_test() {
    let match_phrase_test_test = MatchPhrase::new().field("brand")
        .value("DPH")
        .analyzer("english")
        .slop(2u64)
        .zero_terms_query("all");
    
    let query = Query::new()
        .source(vec!["brand"])
        .query(match_phrase_test_test);

    let json = json!(query);
    let json_str = json!({
        "query":{
            "match_phrase":{
                "brand":{
                    "query": "DPH",
                    "analyzer": "english",
                    "slop": 2,
                    "zero_terms_query": "all"
                }
            }
        },
        "_source":["brand"]
    });
    
    assert_eq!(json_str, json);
}

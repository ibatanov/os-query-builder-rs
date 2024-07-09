use serde_json::json;
use os_query_builder_rs::full_text::simple_query_string::SimpleQueryString;
use os_query_builder_rs::model::Query;

#[test]
fn simple_query_string_test() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .field("brand")
        .query("Сальники+")
        .analyze_wildcard(true)
        .fuzzy_max_expansions(60u64)
        .fuzzy_transpositions(true)
        .lenient(true)
        .minimum_should_match("2")
        .auto_generate_synonyms_phrase_query(false);

    let actual_json = json!(simple_query_string);
    let excepted_json = json!({
        "analyze_wildcard": true,
        "auto_generate_synonyms_phrase_query": false,
        "fields": [
            "catalogue_name",
            "brand"
        ],
        "fuzzy_max_expansions": 60,
        "fuzzy_transpositions": true,
        "lenient": true,
        "minimum_should_match": "2",
        "query": "Сальники+"
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn query_simple_query_string_test() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .field("brand")
        .query("Сальники+")
        .analyze_wildcard(true)
        .fuzzy_max_expansions(60u64)
        .fuzzy_transpositions(true)
        .lenient(true)
        .minimum_should_match("2")
        .auto_generate_synonyms_phrase_query(false);

    let query = Query::new().query(simple_query_string).size(20usize);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "simple_query_string": {
                "analyze_wildcard": true,
                "auto_generate_synonyms_phrase_query": false,
                "fields": [
                    "catalogue_name",
                    "brand"
                ],
                "fuzzy_max_expansions": 60,
                "fuzzy_transpositions": true,
                "lenient": true,
                "minimum_should_match": "2",
                "query": "Сальники+"
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, excepted_json);
}
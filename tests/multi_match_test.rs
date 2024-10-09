use serde_json::json;
use os_query_builder_rs::{
    full_text::multi_match::MultiMatch,
    misc::{
        operator::Operator,
        r#type::Type,
        zero_terms_query::ZeroTermsQuery
    }
};


#[test]
fn multi_match_test() {
    let multi_match = MultiMatch::new()
        .fields(vec!["brands", "articles"])
        .value("oc47")
        .operator(Operator::And)
        .query_type(Type::BestFields)
        .boost(2)
        .minimum_should_match("90%");
    let json_str = json!({"boost":2.0,"fields":["brands","articles"],"minimum_should_match":"90%","operator":"and","query":"oc47","type":"best_fields"});
    let json = json!(multi_match);
    assert_eq!(json_str, json);
}



#[test]
fn multi_match_all_fields_test() {
    let multi_match = MultiMatch::new()
        .fields(vec!["brands", "articles"])
        .value("oc47")
        .operator(Operator::And)
        .query_type(Type::BestFields)
        .boost(2)
        .minimum_should_match("90%")
        .lenient(true)
        .zero_terms_query(ZeroTermsQuery::All)
        .fuzzy_transpositions(false)
        .auto_generate_synonyms_phrase_query(true);
    let json_str = json!({
        "boost":2.0,
        "fields":["brands","articles"],
        "minimum_should_match":"90%",
        "operator":"and",
        "query":"oc47",
        "type":"best_fields",
        "lenient": true,
        "zero_terms_query": "all",
        "fuzzy_transpositions": false,
        "auto_generate_synonyms_phrase_query": true
    });
    let json = json!(multi_match);
    assert_eq!(json_str, json);
}
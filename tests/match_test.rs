use serde_json::json;
use os_query_builder_rs::{
    full_text::r#match::Match,
    model::Query,
    misc::{
        operator::Operator,
        fuzziness::Fuzziness,
        zero_terms_query::ZeroTermsQuery
    }
};

#[test]
fn query_test() {
    let match_test = Match::new().field("brand").value("My Brand");

    let query = Query::new().source(vec!["test"]).query(match_test);

    let json_str = json!({"query":{"match":{"brand":{"query":"My Brand"}}},"_source":["test"]});
    let json = json!(query);
    assert_eq!(json_str, json);
}

#[test]
fn match_test() {
    let mq = Match::new()
        .field("brands")
        .value("My Brand")
        .boost(2f64)
        .operator(Operator::Or);

    let json_str = json!({"brands":{"boost":2.0,"operator":"or","query":"My Brand"}});
    let json = serde_json::to_value(mq).unwrap();
    assert_eq!(json_str, json);
}

#[test]
fn match_test_with_zero_terms_query_all() {
    let mq = Match::new()
        .field("brands")
        .value("My Brand")
        .boost(2f64)
        .operator(Operator::Or)
        .zero_terms_query(ZeroTermsQuery::All);

    let json_str = json!({
        "brands": {
            "boost":2.0,
            "operator":"or",
            "zero_terms_query": "all",
            "query":"My Brand"
        }
    });
    let json = serde_json::to_value(mq).unwrap();
    assert_eq!(json_str, json);
}


#[test]
fn match_test_with_zero_terms_query_none() {
    let mq = Match::new()
        .field("brands")
        .value("My Brand")
        .boost(2f64)
        .operator(Operator::Or)
        .zero_terms_query(ZeroTermsQuery::None);

    let json_str = json!({
        "brands": {
            "boost":2.0,
            "operator":"or",
            "zero_terms_query": "none",
            "query":"My Brand"
        }
    });
    let json = json!(mq);
    assert_eq!(json_str, json);
}


#[test]
fn match_with_all_field() {
    let mq = Match::new()
        .field("brands")
        .value("My Brand")
        .boost(2f64)
        .operator(Operator::Or)
        .max_expansions(50u64)
        .fuzziness(Fuzziness::Auto)
        .fuzzy_transpositions(false)
        .lenient(true)
        .zero_terms_query(ZeroTermsQuery::None);

    let json_str = json!({
        "brands": {
            "boost":2.0,
            "operator":"or",
            "zero_terms_query": "none",
            "query":"My Brand",
            "max_expansions": 50,
            "fuzziness": "AUTO",
            "fuzzy_transpositions": false,
            "lenient": true
        }
    });
    let json = json!(mq);
    assert_eq!(json_str, json);
}

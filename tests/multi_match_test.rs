use serde_json::json;

use os_query_builder_rs::full_text::multi_match::MultiMatch;
use os_query_builder_rs::full_text::r#match::Match;
use os_query_builder_rs::misc::operator::Operator;
use os_query_builder_rs::misc::r#type::Type;
use os_query_builder_rs::model::Query;


#[test]
fn query_test() {
    let match_test = Match::new().field("brand").value("My Brand");

    let query = Query::new().source(vec!["test"]).query(match_test);

    let json_str = json!({"query":{"match":{"brand":{"query":"My Brand"}}},"_source":["test"]});
    let json = serde_json::to_value(query).unwrap();
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
fn multi_match_test() {
    let multi_match = MultiMatch::new()
        .fields(vec!["brands", "articles"])
        .value("oc47")
        .operator(Operator::And)
        .query_type(Type::BestFields)
        .boost(2)
        .minimum_should_match("90%");
    let json_str = json!({"boost":2.0,"fields":["brands","articles"],"minimum_should_match":"90%","operator":"and","query":"oc47","type":"best_fields"});
    let json = serde_json::to_value(multi_match).unwrap();
    assert_eq!(json_str, json);
}
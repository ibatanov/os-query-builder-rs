use serde_json::json;
use os_query_builder_rs::{
    misc::fuzziness::Fuzziness,
    full_text::intervals::{
        interval_rule::IntervalRule,
        rules::{
            fuzzy::FuzzyRule,
            prefix::PrefixRule,
            r#match::MatchRule,
            wildcard::WildcardRule
        }
    }
};


#[test]
fn all_field_math_rule_without_filter() {
    let match_rule = MatchRule::new()
        .query("query")
        .analyzer("standard")
        .max_gaps(50)
        .ordered(true)
        .use_field("my_field");

    let actual_json = json!(match_rule);
    let excepted_json = json!({
            "query": "query",
            "analyzer":"standard",
            "max_gaps": 50,
            "ordered": true,
            "use_field": "my_field"
    });

    assert_eq!(actual_json, excepted_json);
}
#[test]
fn all_field_math_rule_in_enum_without_filter() {
    let match_rule = IntervalRule::Match(MatchRule::new()
        .query("query")
        .analyzer("standard")
        .max_gaps(50)
        .ordered(true)
        .use_field("my_field"));

    let actual_json = json!(match_rule);
    let excepted_json = json!({
       "match": {
            "query": "query",
            "analyzer":"standard",
            "max_gaps": 50,
            "ordered": true,
            "use_field": "my_field"
        }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn all_field_prefix_rule_without_filter() {
    let prefix_rule = PrefixRule::new()
        .prefix("prefix")
        .analyzer("standard")
        .use_field("my_field");

    let actual_json = json!(prefix_rule);
    let excepted_json = json!({
            "prefix": "prefix",
            "analyzer":"standard",
            "use_field": "my_field"
    });

    assert_eq!(actual_json, excepted_json);
}
#[test]
fn all_field_prefix_rule_in_enum_without_filter() {
    let match_rule = IntervalRule::Prefix(PrefixRule::new()
        .prefix("prefix")
        .analyzer("standard")
        .use_field("my_field"));

    let actual_json = json!(match_rule);
    let excepted_json = json!({
       "prefix": {
            "prefix": "prefix",
            "analyzer":"standard",
            "use_field": "my_field"
        }
    });

    assert_eq!(actual_json, excepted_json);
}



#[test]
fn all_field_wildcard_rule_without_filter() {
    let prefix_rule = WildcardRule::new()
        .pattern("pattern")
        .analyzer("standard")
        .use_field("my_field");

    let actual_json = json!(prefix_rule);
    let excepted_json = json!({
            "pattern": "pattern",
            "analyzer":"standard",
            "use_field": "my_field"
    });

    assert_eq!(actual_json, excepted_json);
}
#[test]
fn all_field_wildcard_rule_in_enum_without_filter() {
    let match_rule = IntervalRule::Wildcard(WildcardRule::new()
        .pattern("pattern")
        .analyzer("standard")
        .use_field("my_field"));

    let actual_json = json!(match_rule);
    let excepted_json = json!({
       "wildcard": {
            "pattern": "pattern",
            "analyzer":"standard",
            "use_field": "my_field"
        }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn all_field_fuzzy_rule_without_filter() {
    let prefix_rule = FuzzyRule::new()
        .term("term")
        .analyzer("standard")
        .fuzziness(Fuzziness::Auto)
        .transpositions(true)
        .prefix_length(50)
        .use_field("my_field");

    let actual_json = json!(prefix_rule);
    let excepted_json = json!({
            "term": "term",
            "analyzer":"standard",
            "use_field": "my_field",
            "fuzziness": "AUTO",
            "transpositions": true,
            "prefix_length": 50
    });

    assert_eq!(actual_json, excepted_json);
}
#[test]
fn all_field_fuzzy_rule_in_enum_without_filter() {
    let match_rule = IntervalRule::Fuzzy(FuzzyRule::new()
        .term("term")
        .analyzer("standard")
        .use_field("my_field")
        .fuzziness(Fuzziness::Auto)
        .transpositions(true)
        .prefix_length(50)
    );

    let actual_json = json!(match_rule);
    let excepted_json = json!({
       "fuzzy": {
            "term": "term",
            "analyzer":"standard",
            "use_field": "my_field",
            "fuzziness": "AUTO",
            "transpositions": true,
            "prefix_length": 50
        }
    });

    assert_eq!(actual_json, excepted_json);
}
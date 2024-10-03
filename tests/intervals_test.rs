use serde_json::json;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::misc::script::Script;
use os_query_builder_rs::full_text::intervals::{
  interval::Intervals,
  interval_rule::IntervalRule,
  rules::{
      any_of::AnyOfRule,
      r#match::MatchRule,
      all_of::AllOfRule,
      filter::FilterRule,
      fuzzy::FuzzyRule,
      prefix::PrefixRule,
      wildcard::WildcardRule
    }
};


#[test]
fn first_example_from_docs_test() {
    let match_any_of_1 = MatchRule::new().query("hash map");
    let match_any_of_2 = MatchRule::new().query("hash table");
    let any_of_rule = AnyOfRule::new().intervals(vec![match_any_of_2, match_any_of_1]);
    let match_all_of = MatchRule::new()
        .query("key-value pairs")
        .max_gaps(0)
        .ordered(true);
    let all_of_rule = AllOfRule::new()
        .ordered(true)
        .intervals(vec![
            IntervalRule::Match(match_all_of),
            IntervalRule::AnyOf(any_of_rule)
        ]);

    let intervals = Intervals::new()
        .field("title")
        .intervals(all_of_rule);
    let query = Query::new().query(intervals);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "intervals": {
          "title": {
            "all_of": {
              "ordered": true,
              "interval": [
                {
                  "match": {
                    "query": "key-value pairs",
                    "max_gaps": 0,
                    "ordered": true
                  }
                },
                {
                  "any_of": {
                    "interval": [
                      {
                        "match": {
                          "query": "hash table"
                        }
                      },
                      {
                        "match": {
                          "query": "hash map"
                        }
                      }
                    ]
                  }
                }
              ]
            }
          }
        }
      }
    });

    assert_eq! (actual_json, excepted_json);
}


#[test]
fn second_example_from_docs_test() {

    let filter_match_rule = MatchRule::new().query("efficiently");
    let interval_match_rule = MatchRule::new()
        .query("pairs hash")
        .max_gaps(5)
        .filter(FilterRule::NotContaining(IntervalRule::Match(filter_match_rule)));

    let intervals = Intervals::new()
        .field("title")
        .intervals(interval_match_rule);
    let query = Query::new().query(intervals);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "intervals" : {
          "title" : {
            "match" : {
              "query" : "pairs hash",
              "max_gaps" : 5,
              "filter" : {
                "not_containing" : {
                  "match" : {
                    "query" : "efficiently"
                  }
                }
              }
            }
          }
        }
      }
    });

    assert_eq! (actual_json, excepted_json);
}


#[test]
fn example_with_script_from_docs_test() {

    let script_value = Script::new()
        .source("interval.start > 5 && interval.end < 8 && interval.gaps == 0");
    let interval_match_rule = MatchRule::new()
        .query("map hash")
        .filter(FilterRule::Script(script_value));

    let intervals = Intervals::new()
        .field("title")
        .intervals(interval_match_rule);
    let query = Query::new().query(intervals);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "intervals" : {
          "title" : {
            "match" : {
              "query" : "map hash",
              "filter" : {
                "script" : {
                  "source" : "interval.start > 5 && interval.end < 8 && interval.gaps == 0"
                }
              }
            }
          }
        }
      }
    });

    assert_eq! (actual_json, excepted_json);
}


#[test]
fn example_minimization_from_docs_test() {

    let interval_match_rule = MatchRule::new()
        .query("a c");
    let interval_match_rule = MatchRule::new()
        .query("d")
        .filter(FilterRule::ContainedBy(IntervalRule::Match(interval_match_rule)));

    let intervals = Intervals::new()
        .field("my_text")
        .intervals(interval_match_rule);
    let query = Query::new().query(intervals);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "intervals" : {
          "my_text" : {
            "match" : {
              "query" : "d",
              "filter" : {
                "contained_by" : {
                  "match" : {
                    "query" : "a c"
                  }
                }
              }
            }
          }
        }
      }
    });

    assert_eq! (actual_json, excepted_json);
}


#[test]
fn interval_with_all_of_and_nested_any_of_with_other_rules_docs_test() {

    let prefix_rule = PrefixRule::new().prefix("a c");
    let match_rule = MatchRule::new().query("d");
    let fuzzy_rule = FuzzyRule::new().term("term");
    let wildcard_rule = WildcardRule::new().pattern("d");


    // todo сделать публичные интервалы?
    let a = vec![IntervalRule::Match(match_rule), prefix_rule.into(), fuzzy_rule.into(), wildcard_rule.into()];
    let any_of_rule = AnyOfRule::new().intervals(a);

    let all_of_rule = AllOfRule::new().intervals(vec![any_of_rule]);
    let intervals = Intervals::new()
        .field("my_text")
        .intervals(all_of_rule);
    let query = Query::new().query(intervals);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "intervals" : {
          "my_text" : {
            "all_of" : {
              "interval": [
                {
                  "any_of": {
                    "interval": [
                      {
                        "match": {
                          "query": "d"
                        }
                      },
                      {
                        "prefix": {
                          "prefix": "a c"
                        }
                      },
                      {
                        "fuzzy": {
                          "term": "term"
                        }
                      },
                      {
                        "wildcard": {
                          "pattern":"d"
                        }
                      }
                    ]
                  }
                }

              ]
            }
          }
        }
      }
    });


    assert_eq! (actual_json, excepted_json);
}
use serde_json::json;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::terms_set::TermsSet;

#[test]
fn first_example_from_docs_test() {
    let terms_set = TermsSet::new()
        .field("classes")
        .terms(vec!["CS101", "CS102", "MATH101"])
        .minimum_should_match_field("min_required");
    let query = Query::new().query(terms_set);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "terms_set": {
          "classes": {
            "terms": [ "CS101", "CS102", "MATH101" ],
            "minimum_should_match_field": "min_required"
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

#[test]
fn first_example_from_docs_test_with_boost() {
    let terms_set = TermsSet::new()
        .field("classes")
        .terms(vec!["CS101", "CS102", "MATH101"])
        .minimum_should_match_field("min_required")
        .boost(0.5f64);
    let query = Query::new().query(terms_set);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "terms_set": {
          "classes": {
            "terms": [ "CS101", "CS102", "MATH101" ],
            "minimum_should_match_field": "min_required",
            "boost": 0.5
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn script_example_from_docs_test_with_boost() {
    let terms_set = TermsSet::new()
        .field("classes")
        .terms(vec!["CS101", "CS102", "MATH101"])
        .minimum_should_match_script("Math.min(params.num_terms, doc['min_required'].value)")
        .boost(0.5f64);
    let query = Query::new().query(terms_set);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "terms_set": {
          "classes": {
            "terms": [ "CS101", "CS102", "MATH101" ],
            "minimum_should_match_script": {
              "source": "Math.min(params.num_terms, doc['min_required'].value)"
            },
            "boost": 0.5
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn script_script_overwrite_field() {
    let terms_set = TermsSet::new()
        .field("classes")
        .terms(vec!["CS101", "CS102", "MATH101"])
        .minimum_should_match_field("min_required")
        .minimum_should_match_script("Math.min(params.num_terms, doc['min_required'].value)")
        .boost(0.5f64);
    let query = Query::new().query(terms_set);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "terms_set": {
          "classes": {
            "terms": [ "CS101", "CS102", "MATH101" ],
            "minimum_should_match_script": {
              "source": "Math.min(params.num_terms, doc['min_required'].value)"
            },
            "boost": 0.5
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}


#[test]
fn script_field_overwrite_script() {
    let terms_set = TermsSet::new()
        .field("classes")
        .terms(vec!["CS101", "CS102", "MATH101"])
        .minimum_should_match_script("Math.min(params.num_terms, doc['min_required'].value)")
        .minimum_should_match_field("min_required")
        .boost(0.5f64);
    let query = Query::new().query(terms_set);

    let actual_json = json!(query);
    let excepted_json = json!({
      "query": {
        "terms_set": {
          "classes": {
            "terms": [ "CS101", "CS102", "MATH101" ],
            "minimum_should_match_field": "min_required",
            "boost": 0.5
          }
        }
      }
    });

    assert_eq!(actual_json, excepted_json);
}

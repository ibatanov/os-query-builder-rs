use serde_json::json;
use os_query_builder_rs::model::Query;
use os_query_builder_rs::term::term::Term;

#[test]
fn term_test() {
    let term = Term::new("brand_name", "FIAT");
    let json_actual = json!(term);

    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_test_with_value_is_int() {
    let term = Term::new("brand_id", 1 as i64);
    let json_actual = json!(term);

    let json_expected = json!({
        "brand_id": {
            "value": 1,
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_test_with_value_is_float64() {
    let term = Term::new("brand_id", 1.3211);
    let json_actual = json!(term);

    let json_expected = json!({
        "brand_id": {
            "value": 1.3211,
        }
    });
    assert_eq!(json_expected, json_actual);
}


#[test]
fn term_test_with_value_is_float32() {

    let value = 24334.1232131 as f32;
    let term = Term::new("brand_id", value);
    let json_actual = json!(term);

    let json_expected = json!({
        "brand_id": {
            "value": value,
        }
    });
    assert_eq!(json_expected, json_actual);
}

#[test]
fn term_with_boost_test() {
    let term = Term::new("brand_name", "FIAT").boost(0.65);

    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "boost": 0.65
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_with_case_insensitive_test() {
    let term = Term::new("brand_name", "FIAT").case_insensitive(true);

    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "case_insensitive": true
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn term_with_boost_and_case_insensitive_test() {
    let term = Term::new("brand_name", "FIAT")
        .case_insensitive(true)
        .boost(0.6);

    let json_actual = json!(term);
    let json_expected = json!({
        "brand_name": {
            "value": "FIAT",
            "case_insensitive": true,
            "boost": 0.6
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn query_term_with_boost_and_case_insensitive_test() {
    let term = Term::new("article", "43935055")
        .case_insensitive(true)
        .boost(0.6);

    let query = Query::new().query(term);

    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
            "term": {
                "article": {
                    "boost":0.6,
                    "case_insensitive":true,
                    "value":"43935055"
                }
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
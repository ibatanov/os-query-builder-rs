use serde_json::json;
use os_query_builder_rs::term::term::Term;
use os_query_builder_rs::term::terms::{Terms, TermsLookup};
use os_query_builder_rs::{
    compound_query::bool::Bool,
    model::Query,
    misc::query_field::QueryField
};
use os_query_builder_rs::full_text::{
    r#match::Match,
    match_phrase::MatchPhrase,
    match_phrase_prefix::MatchPhrasePrefix,
    simple_query_string::SimpleQueryString,
    multi_match::MultiMatch
};
use os_query_builder_rs::misc::operator::Operator;
use os_query_builder_rs::misc::r#type::Type;


#[test]
fn compound_bool_query_with_one_match_must() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![QueryField::Match(match_value)]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
              "match": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_in_must() {
    let match_value_1 = Match::new().field("name").value("Деталь");
    let match_value_2 = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
                "match": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_must() {
    let match_phrase = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![match_phrase]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
              "match_phrase": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_phrase_in_must() {
    let match_value_1 = MatchPhrase::new().field("name").value("Деталь");
    let match_value_2 = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
                "match_phrase": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match_phrase": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_prefix_must() {
    let match_phrase_prefix = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);

    let boolean = Bool::new().must(vec![match_phrase_prefix]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_match_phrase_prefix_must() {
    let match_phrase_prefix_1 = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    let match_phrase_prefix_2 = MatchPhrasePrefix::new()
        .field("article")
        .query("0029972147");

    let boolean = Bool::new().must(vec![match_phrase_prefix_1, match_phrase_prefix_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            },
            {
                "match_phrase_prefix":{
                    "article": {
                        "query":"0029972147"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_simple_query_string_must() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+");

    let boolean = Bool::new().must(vec![simple_query_string]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must": [
            {
                "simple_query_string": {
                    "fields": [ "catalogue_name" ],
                    "query": "Сальники+"
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_simple_query_string_must() {
    let simple_query_string_1 = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+двигателя");
    let simple_query_string_2 = SimpleQueryString::new()
        .fields(vec!["brand"])
        .query("FIAT|ALFA");

    let boolean = Bool::new().must(vec![simple_query_string_1, simple_query_string_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
        "bool": {
            "must": [
                {
                    "simple_query_string": {
                        "fields": [
                            "catalogue_name"
                        ],
                        "query": "Сальники+двигателя"
                    }
                },
                {
                    "simple_query_string": {
                        "fields": [
                            "brand"
                        ],
                        "query": "FIAT|ALFA"
                    }
                }
            ]
        }
      },
      "size": 20
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_term_in_must() {
    let term = Term::new("name", "Деталь");
    let boolean = Bool::new().must(vec![term]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_term_in_must() {
    let term_1 = Term::new("name", "Деталь");
    let term_2 = Term::new("product_id", "10407391");
    let boolean = Bool::new().must(vec![term_1, term_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    },
                    {
                        "term": {
                            "product_id": {
                                "value": "10407391"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_terms_query_in_must() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let boolean = Bool::new().must(vec![terms_query]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_in_must() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_query_2 = Terms::new_with_terms_query("brand_id", vec!["7859", "34"]);
    let boolean = Bool::new().must(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": [
                                "7859",
                                "34"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_lookup_in_must() {
    let terms_lookup_1 = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_1 = Terms::new_with_terms_lookup("brand_id", terms_lookup_1);
    let terms_lookup_2 = TermsLookup::new("sku", "10407391", "product_id");
    let terms_query_2 = Terms::new_with_terms_lookup("product_id", terms_lookup_2);

    let boolean = Bool::new().must(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "terms": {
                            "product_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "product_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_in_must() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);

    let boolean = Bool::new().must(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_must() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let match_value = Match::new().field("brand").value("FIAT");

    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);


    let boolean = Bool::new().must(vec![
        QueryField::Terms(terms_query_1),
        QueryField::Terms(terms_query_2),
        QueryField::Match(match_value),
    ]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_must_with_minimum_should_match() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let match_value = Match::new().field("brand").value("FIAT");


    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);


    let boolean = Bool::new()
        .must(vec![
            QueryField::Terms(terms_query_1),
            QueryField::Terms(terms_query_2),
            QueryField::Match(match_value),
        ])
        .minimum_should_match(3usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ],
                "minimum_should_match": 3
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}



#[test]
fn compound_bool_query_with_one_match_must_not() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must_not(vec![QueryField::Match(match_value)]);

    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
              "match": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_in_must_not() {
    let match_value_1 = Match::new().field("name").value("Деталь");
    let match_value_2 = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must_not(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
                "match": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_must_not() {
    let match_phrase = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().must_not(vec![match_phrase]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
              "match_phrase": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_phrase_in_must_not() {
    let match_value_1 = MatchPhrase::new().field("name").value("Деталь");
    let match_value_2 = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().must_not(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
                "match_phrase": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match_phrase": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_prefix_must_not() {
    let match_phrase_prefix = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);

    let boolean = Bool::new().must_not(vec![match_phrase_prefix]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_match_phrase_prefix_must_not() {
    let match_phrase_prefix_1 = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    let match_phrase_prefix_2 = MatchPhrasePrefix::new()
        .field("article")
        .query("0029972147");

    let boolean = Bool::new().must_not(vec![match_phrase_prefix_1, match_phrase_prefix_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            },
            {
                "match_phrase_prefix":{
                    "article": {
                        "query":"0029972147"
                    }
                }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_simple_query_string_must_not() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+");

    let boolean = Bool::new().must_not(vec![simple_query_string]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "must_not": [
            {
                "simple_query_string": {
                    "fields": [ "catalogue_name" ],
                    "query": "Сальники+"
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_simple_query_string_must_not() {
    let simple_query_string_1 = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+двигателя");
    let simple_query_string_2 = SimpleQueryString::new()
        .fields(vec!["brand"])
        .query("FIAT|ALFA");

    let boolean = Bool::new().must_not(vec![simple_query_string_1, simple_query_string_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
        "bool": {
            "must_not": [
                {
                    "simple_query_string": {
                        "fields": [
                            "catalogue_name"
                        ],
                        "query": "Сальники+двигателя"
                    }
                },
                {
                    "simple_query_string": {
                        "fields": [
                            "brand"
                        ],
                        "query": "FIAT|ALFA"
                    }
                }
            ]
        }
      },
      "size": 20
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_term_in_must_not() {
    let term = Term::new("name", "Деталь");
    let boolean = Bool::new().must_not(vec![term]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_term_in_must_not() {
    let term_1 = Term::new("name", "Деталь");
    let term_2 = Term::new("product_id", "10407391");
    let boolean = Bool::new().must_not(vec![term_1, term_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    },
                    {
                        "term": {
                            "product_id": {
                                "value": "10407391"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_terms_query_in_must_not() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let boolean = Bool::new().must_not(vec![terms_query]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_in_must_not() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_query_2 = Terms::new_with_terms_query("brand_id", vec!["7859", "34"]);
    let boolean = Bool::new().must_not(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": [
                                "7859",
                                "34"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_lookup_in_must_not() {
    let terms_lookup_1 = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_1 = Terms::new_with_terms_lookup("brand_id", terms_lookup_1);

    let terms_lookup_2 = TermsLookup::new("sku", "10407391", "product_id");
    let terms_query_2 = Terms::new_with_terms_lookup("product_id", terms_lookup_2);

    let boolean = Bool::new().must_not(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "terms": {
                            "product_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "product_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_in_must_not() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);

    let boolean = Bool::new().must_not(vec![terms_query_1, terms_query_2]);

    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_must_not() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let match_value = Match::new().field("brand").value("FIAT");

    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);


    let boolean = Bool::new().must_not(vec![
        QueryField::Terms(terms_query_1),
        QueryField::Terms(terms_query_2),
        QueryField::Match(match_value),
    ]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_must_not_with_minimum_should_match() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let match_value = Match::new().field("brand").value("FIAT");

    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);



    let boolean = Bool::new()
        .must_not(vec![
            QueryField::Terms(terms_query_1),
            QueryField::Terms(terms_query_2),
            QueryField::Match(match_value),
        ])
        .minimum_should_match(2usize);

    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ],
                "minimum_should_match": 2
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}



#[test]
fn compound_bool_query_with_one_match_filter() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().filter(vec![QueryField::Match(match_value)]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
              "match": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_in_filter() {
    let match_value_1 = Match::new().field("name").value("Деталь");
    let match_value_2 = Match::new().field("brand").value("FIAT");

    let boolean = Bool::new().filter(vec![match_value_1, match_value_2]);

    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
                "match": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_filter() {
    let match_phrase = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().filter(vec![match_phrase]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
              "match_phrase": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_phrase_in_filter() {
    let match_value_1 = MatchPhrase::new().field("name").value("Деталь");
    let match_value_2 = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().filter(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
                "match_phrase": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match_phrase": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_prefix_filter() {
    let match_phrase_prefix = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);

    let boolean = Bool::new().filter(vec![match_phrase_prefix]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_match_phrase_prefix_filter() {
    let match_phrase_prefix_1 = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    let match_phrase_prefix_2 = MatchPhrasePrefix::new()
        .field("article")
        .query("0029972147");

    let boolean = Bool::new().filter(vec![match_phrase_prefix_1, match_phrase_prefix_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            },
            {
                "match_phrase_prefix":{
                    "article": {
                        "query":"0029972147"
                    }
                }
            }
          ]
        }
      }
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_simple_query_string_filter() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+");

    let boolean = Bool::new().filter(vec![simple_query_string]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "filter": [
            {
                "simple_query_string": {
                    "fields": [ "catalogue_name" ],
                    "query": "Сальники+"
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_simple_query_string_filter() {
    let simple_query_string_1 = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+двигателя");
    let simple_query_string_2 = SimpleQueryString::new()
        .fields(vec!["brand"])
        .query("FIAT|ALFA");

    let boolean = Bool::new().filter(vec![simple_query_string_1, simple_query_string_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
        "bool": {
            "filter": [
                {
                    "simple_query_string": {
                        "fields": [
                            "catalogue_name"
                        ],
                        "query": "Сальники+двигателя"
                    }
                },
                {
                    "simple_query_string": {
                        "fields": [
                            "brand"
                        ],
                        "query": "FIAT|ALFA"
                    }
                }
            ]
        }
      },
      "size": 20
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_term_in_filter() {
    let term = Term::new("name", "Деталь");
    let boolean = Bool::new().filter(vec![term]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_term_in_filter() {
    let term_1 = Term::new("name", "Деталь");
    let term_2 = Term::new("product_id", "10407391");

    let boolean = Bool::new().filter(vec![term_1, term_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    },
                    {
                        "term": {
                            "product_id": {
                                "value": "10407391"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_terms_query_in_filter() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);

    let boolean = Bool::new().filter(vec![terms_query]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_in_filter() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_query_2 = Terms::new_with_terms_query("brand_id", vec!["7859", "34"]);

    let boolean = Bool::new().filter(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": [
                                "7859",
                                "34"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_lookup_in_filter() {
    let terms_lookup_1 = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_1 = Terms::new_with_terms_lookup("brand_id", terms_lookup_1);

    let terms_lookup_2 = TermsLookup::new("sku", "10407391", "product_id");
    let terms_query_2 = Terms::new_with_terms_lookup("product_id", terms_lookup_2);


    let boolean = Bool::new().filter(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "terms": {
                            "product_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "product_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_in_filter() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);

    let boolean = Bool::new().filter(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_filter() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);
    let match_value = Match::new().field("brand").value("FIAT");

    let boolean = Bool::new().filter(vec![
        QueryField::Terms(terms_query_1),
        QueryField::Terms(terms_query_2),
        QueryField::Match(match_value),
    ]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_filter_with_minimum_should_match() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);
    let match_value = Match::new().field("brand").value("FIAT");

    let boolean = Bool::new()
        .filter(vec![
            QueryField::Terms(terms_query_1),
            QueryField::Terms(terms_query_2),
            QueryField::Match(match_value),
        ])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ],
                "minimum_should_match": 2
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}



#[test]
fn compound_bool_query_with_one_match_should() {
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().should(vec![QueryField::Match(match_value)]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
              "match": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_math_in_should() {
    let match_value_1 = Match::new().field("name").value("Деталь");
    let match_value_2 = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().should(vec![match_value_1, match_value_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
                "match": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_should() {
    let match_phrase = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().should(vec![match_phrase]).minimum_should_match(1usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
              "match_phrase": {
                "brand": {
                    "query": "FIAT"
                }
              }
            }
          ],
          "minimum_should_match": 1
        }
      }
    });
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_match_phrase_should() {
    let match_value_1 = MatchPhrase::new().field("name").value("Деталь");
    let match_value_2 = MatchPhrase::new().field("brand").value("FIAT");
    let boolean = Bool::new().should(vec![match_value_1, match_value_2])
        .minimum_should_match(1usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
                "match_phrase": {
                    "name": {
                        "query": "Деталь"
                    }
                }
            },
            {
                "match_phrase": {
                    "brand": {
                        "query": "FIAT"
                    }
                }
            }
          ],
          "minimum_should_match": 1
        }
      }
    });
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_match_phrase_prefix_should() {
    let match_phrase_prefix = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);

    let boolean = Bool::new()
        .should(vec![match_phrase_prefix])
        .minimum_should_match(1usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            }
          ],
          "minimum_should_match": 1
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_match_phrase_prefix_should() {
    let match_phrase_prefix_1 = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);
    let match_phrase_prefix_2 = MatchPhrasePrefix::new()
        .field("article")
        .query("0029972147");

    let boolean = Bool::new()
        .should(vec![match_phrase_prefix_1, match_phrase_prefix_2])
        .minimum_should_match(1usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
                "match_phrase_prefix":{
                    "brand":{
                        "query": "DPH",
                        "analyzer": "stop",
                        "slop": 3,
                        "max_expansions": 60
                    }
                }
            },
            {
                "match_phrase_prefix":{
                    "article": {
                        "query":"0029972147"
                    }
                }
            }
          ],
          "minimum_should_match": 1
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_simple_query_string_should() {
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+");

    let boolean = Bool::new().should(vec![simple_query_string]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
      "size": 20,
      "query": {
        "bool": {
          "should": [
            {
                "simple_query_string": {
                    "fields": [ "catalogue_name" ],
                    "query": "Сальники+"
                }
            }
          ]
        }
      }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_simple_query_string_should() {
    let simple_query_string_1 = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+двигателя");
    let simple_query_string_2 = SimpleQueryString::new()
        .fields(vec!["brand"])
        .query("FIAT|ALFA");

    let boolean = Bool::new()
        .should(vec![simple_query_string_1, simple_query_string_2])
        .minimum_should_match(1usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "query": {
        "bool": {
            "minimum_should_match": 1,
            "should": [
                {
                    "simple_query_string": {
                        "fields": [
                            "catalogue_name"
                        ],
                        "query": "Сальники+двигателя"
                    }
                },
                {
                    "simple_query_string": {
                        "fields": [
                            "brand"
                        ],
                        "query": "FIAT|ALFA"
                    }
                }
            ]
        }
      },
      "size": 20
    });
    
    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_term_in_should() {
    let term = Term::new("name", "Деталь");
    let boolean = Bool::new().should(vec![term]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_term_in_should() {
    let term_1 = Term::new("name", "Деталь");
    let term_2 = Term::new("product_id", "10407391");
    let boolean = Bool::new().should(vec![term_1, term_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    },
                    {
                        "term": {
                            "product_id": {
                                "value": "10407391"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_one_terms_query_in_should() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let boolean = Bool::new().should(vec![terms_query]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_in_should() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_query_2 = Terms::new_with_terms_query("brand_id", vec!["7859", "34"]);

    let boolean = Bool::new().should(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": [
                                "7859",
                                "34"
                            ]
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_lookup_in_should() {
    let terms_lookup_1 = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_1 = Terms::new_with_terms_lookup("brand_id", terms_lookup_1);
    let terms_lookup_2 = TermsLookup::new("sku", "10407391", "product_id");
    let terms_query_2 = Terms::new_with_terms_lookup("product_id", terms_lookup_2);

    let boolean = Bool::new().should(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "terms": {
                            "product_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "product_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_in_should() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);

    let boolean = Bool::new().should(vec![terms_query_1, terms_query_2]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_should() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);
    let match_value = Match::new().field("brand").value("FIAT");

    let boolean = Bool::new().should(vec![
        QueryField::Terms(terms_query_1),
        QueryField::Terms(terms_query_2),
        QueryField::Match(match_value),
    ]);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ]
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}
#[test]
fn compound_bool_query_with_several_terms_query_and_terms_lookup_and_match_in_should_with_minimum_should_match() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_2 = Terms::new_with_terms_lookup("brand_id", terms_lookup);
    let match_value = Match::new().field("brand").value("FIAT");

    let boolean = Bool::new()
        .should(vec![
            QueryField::Terms(terms_query_1),
            QueryField::Terms(terms_query_2),
            QueryField::Match(match_value),
        ])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(boolean);

    let json_actual = json!(query);
    let json_expected = json!({
        "size": 20,
        "query": {
            "bool": {
                "should": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    },
                    {
                        "terms": {
                            "brand_id": {
                                "index": "sku",
                                "id": "10407391",
                                "path": "brand_id"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand": {
                                "query": "FIAT"
                            }
                        }
                    }
                ],
                "minimum_should_match": 2
            }
        }
    });

    assert_eq!(json_expected, json_actual);
}


///////////
/// Nested compound boolean query
///////////
#[test]
fn compound_bool_query_with_inner_bools_one_nested_level() {
    let match_value = Match::new().field("brand").value("FIAT");
    let should_boolean = Bool::new().should(vec![match_value]);
    
    let term = Term::new("name", "Деталь");
    let must_boolean = Bool::new().must(vec![term]);
    
    let finish_boolean = Bool::new()
        .should(vec![
            should_boolean,
            must_boolean
        ])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let json_actual = json!(query);
    let json_expected = json!({
      "query": {
        "bool": {
          "minimum_should_match": 2,
          "should": [
            {
              "bool": {
                "should": [
                  {
                    "match": {
                      "brand": {
                        "query": "FIAT"
                      }
                    }
                  }
                ]
              }
            },
            {
              "bool": {
                "must": [
                  {
                    "term": {
                      "name": {
                        "value": "Деталь"
                      }
                    }
                  }
                ]
              }
            }
          ]
        }
      },
      "size": 20
    });

    assert_eq!(json_actual, json_expected);
}
#[test]
fn compound_bool_query_with_inner_bools_two_nested_level() {
    let terms_query_1 = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let first_boolean = Bool::new().filter(vec![terms_query_1]);
    
    let second_boolean = Bool::new().must_not(vec![first_boolean]);
    
    let last_level = Bool::new().must(vec![second_boolean]);
    
    let finish_boolean = Bool::new()
        .should(vec![last_level])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "minimum_should_match": 2,
                "should": [
                    {
                        "bool": {
                            "must": [
                                {
                                    "bool": {
                                        "must_not": [
                                            {
                                                "bool": {
                                                    "filter": [
                                                        {
                                                            "terms": {
                                                                "product_id": [ "10407391", "134249317" ]
                                                            }
                                                        }                             
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "size": 20
    });
    
    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_with_inner_bools_three_nested_level() {
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let first_boolean = Bool::new().filter(vec![terms_query]);
    let second_boolean = Bool::new().should(vec![first_boolean]);
    let third_boolean = Bool::new().should(vec![second_boolean]);
    let last_level = Bool::new().must(vec![third_boolean]);
    
    let finish_boolean = Bool::new()
        .should(vec![last_level])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "minimum_should_match": 2,
                "should": [
                    {
                        "bool": {
                            "must": [
                                {
                                    "bool": {
                                        "should": [
                                            {
                                                "bool": {
                                                    "should": [
                                                        {
                                                            "bool": {
                                                                "filter": [
                                                                    {
                                                                        "terms": {
                                                                            "product_id": [
                                                                                "10407391",
                                                                                "134249317"
                                                                            ]
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}



///////////
/// Complex compound boolean query
/// using combinations of logical operators
///////////
#[test]
fn compound_bool_query_complex_must_should(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .must(vec![terms_query])
        .should(vec![term])
        .minimum_should_match(2usize);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "minimum_should_match": 2,
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "should": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_must_filter(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .must(vec![terms_query])
        .filter(vec![term]);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "filter": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_must_not_filter(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .must_not(vec![terms_query])
        .filter(vec![term]);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "filter": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_must_not_should(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .must_not(vec![terms_query])
        .should(vec![term]);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "should": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_filter_should(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .filter(vec![terms_query])
        .should(vec![term]);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "filter": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "should": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_must_not_must(){
    let terms_query = Terms::new_with_terms_query("product_id", vec!["10407391", "134249317"]);
    let term = Term::new("name", "Деталь");
    let finish_boolean = Bool::new()
        .must_not(vec![terms_query])
        .must(vec![term]);
    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "must_not": [
                    {
                        "terms": {
                            "product_id": [
                                "10407391",
                                "134249317"
                            ]
                        }
                    }
                ],
                "must": [
                    {
                        "term": {
                            "name": {
                                "value": "Деталь"
                            }
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}
#[test]
fn compound_bool_query_complex_double_must_and_must_not(){
    let terms_query = Terms::new_with_terms_query("catalogue_id", vec!["13321", "13322"]);
    let term_article = Term::new("article", "BSG 30-615-013");
    let match_brand = Match::new().field("brand_id").value("2456");
    let finish_boolean = Bool::new()
        .must_not(vec![terms_query])
        .must(vec![QueryField::Term(term_article), QueryField::Match(match_brand)]);

    let query = Query::new().size(20usize).query(finish_boolean);
    
    let actual_json = json!(query);
    let json_expected = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "term": {
                            "article": {
                                "value": "BSG 30-615-013"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand_id": {
                                "query": "2456"
                            }
                        }
                    }
                ],
                "must_not": [
                    {
                        "terms": {
                            "catalogue_id": [
                                "13321",
                                "13322"
                            ]
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, json_expected);
}

#[test]
fn using_all_conditions_test() {
    let terms_query = Terms::new_with_terms_query("catalogue_id", vec!["13321", "13322"]);
    let term_article = Term::new("article", "BSG 30-615-013");
    let match_brand = Match::new().field("brand_id").value("2456");
    let match_phrase_1 = MatchPhrase::new().field("name").value("Деталь");
    let match_phrase_2 = MatchPhrase::new().field("brand").value("BSG");
    let terms_lookup = TermsLookup::new("sku", "10407391", "brand_id");
    let terms_query_lookup = Terms::new_with_terms_lookup("brand_id", terms_lookup);
    let simple_query_string = SimpleQueryString::new()
        .fields(vec!["catalogue_name"])
        .query("Сальники+");
    let multi_match = MultiMatch::new()
        .fields(vec!["brands", "articles"])
        .value("oc47")
        .operator(Operator::And)
        .query_type(Type::BestFields)
        .boost(2)
        .minimum_should_match("90%");


    let match_phrase_prefix = MatchPhrasePrefix::new()
        .field("brand")
        .query("DPH")
        .max_expansions(60u64)
        .analyzer("stop")
        .slop(3u64);

    let nested_boolean = Bool::new()
        .should(vec![
            QueryField::MatchPhrasePrefix(match_phrase_prefix),
            QueryField::MultiMatch(multi_match)
        ])
        .minimum_should_match(1usize)
        .filter(vec![match_phrase_1, match_phrase_2])
        .must(vec![terms_query_lookup])
        .must_not(vec![simple_query_string]);

    let finish_boolean = Bool::new()
        .must_not(vec![terms_query])
        .must(vec![QueryField::Term(term_article),
                   QueryField::Match(match_brand),
                   QueryField::Bool(nested_boolean)
        ]);

    let query = Query::new().size(20usize).query(finish_boolean);

    let actual_json = json!(query);
    let excepted_json = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "term": {
                            "article": {
                                "value": "BSG 30-615-013"
                            }
                        }
                    },
                    {
                        "match": {
                            "brand_id": {
                                "query": "2456"
                            }
                        }
                    },
                    {
                        "bool": {
                            "filter": [
                                {
                                    "match_phrase": {
                                        "name": {
                                            "query": "Деталь"
                                        }
                                    }
                                },
                                {
                                    "match_phrase": {
                                        "brand": {
                                            "query": "BSG"
                                        }
                                    }
                                }
                            ],
                            "minimum_should_match": 1,
                            "must": [
                                {
                                    "terms": {
                                        "brand_id": {
                                            "id": "10407391",
                                            "index": "sku",
                                            "path": "brand_id"
                                        }
                                    }
                                }
                            ],
                            "must_not": [
                                {
                                    "simple_query_string": {
                                        "fields": [
                                            "catalogue_name"
                                        ],
                                        "query": "Сальники+"
                                    }
                                }
                            ],
                            "should": [
                                {
                                    "match_phrase_prefix": {
                                        "brand": {
                                            "analyzer": "stop",
                                            "max_expansions": 60,
                                            "query": "DPH",
                                            "slop": 3
                                        }
                                    }
                                },
                                {
                                    "multi_match": {
                                        "boost": 2.0,
                                        "fields": [
                                            "brands",
                                            "articles"
                                        ],
                                        "minimum_should_match": "90%",
                                        "operator": "and",
                                        "query": "oc47",
                                        "type": "best_fields"
                                    }
                                }
                            ]
                        }
                    }
                ],
                "must_not": [
                    {
                        "terms": {
                            "catalogue_id": [ "13321", "13322" ]
                        }
                    }
                ]
            }
        },
        "size": 20
    });

    assert_eq!(actual_json, excepted_json);
}
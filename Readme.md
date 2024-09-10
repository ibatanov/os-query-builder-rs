# os-query-builder-rs
[![Version](https://img.shields.io/crates/v/os-query-builder-rs)](https://crates.io/crates/os-query-builder-rs)
[![License](https://img.shields.io/crates/l/os-query-builder-rs)](License)

Library for generating queries for Open Search.

- [Installation](#installation)
- [Usage examples](#usage-examples)
  - [MultiMatch](#multimatch)
  - [Term-level](#term-level)
    - [Terms-query](#terms-query)
    - [Terms-lookup](#terms-lookup) 
    - [Term](#term)
  - [Compound-query](#compound-query)
    - [Boolean](#boolean) 
    - [Boosting](#boosting)
    - [ConstantScore](#constant-score)
    - [DisjunctionMax](#disjunction-max)
- [Development plans](#development-plans)

## Installation
```toml
[dependencies]
open-search-query-builder-rs = "0.0.1"
```

### Usage examples

#### MultiMatch
```rust
let multi_match = MultiMatch::new()
            .fields(vec!["brands", "articles"])
            .value("oc47")
            .operator(Operator::And)
            .query_type(Type::BestFields)
            .boost(2)
            .minimum_should_match("90%");

let query = Query::new()
            .source(vec!["test"])
            .query(multi_match);
```

Generates the following request:

```json
{
  "_source": [
    "test"
  ],
  "query": {
    "multi_match": {
      "boost": 2,
      "fields": [
        "brands",
        "articles"
      ],
      "minimum_should_match": 2,
      "operator": "and",
      "query": "oc47",
      "type": "best_fields"
    }
  }
}
```

#### Term-level
##### [Terms query](https://opensearch.org/docs/latest/query-dsl/term/terms/)
```rust
 let terms = Terms::new_with_terms_query("product_id", vec!["128660147","127875288",]).boost(0.7);

let query = Query::new().query(terms)
    .source(vec!["product_id", "brand", "article", "name",]);
```

Generates the following request:
```json
{
  "_source": [
    "product_id",
    "brand",
    "article",
    "name"
  ],
  "query": {
    "terms": {
      "boost": 0.7,
      "product_id": [
        "128660147",
        "127875288"
      ]
    }
  }
}
```

##### [Terms lookup](https://opensearch.org/docs/latest/query-dsl/term/terms/#terms-lookup)
```rust
let terms_lookup = TermsLookup::new("sku", "131356111", "brand_id")
            .routing("brand_id");

let terms = Terms::new_with_terms_lookup("brand_id", terms_lookup)
            .boost(0.7);

let query = Query::new().query(terms);
```

Generates the following request:
```json
{
  "query": {
    "terms": {
      "boost": 0.7,
      "brand_id": {
        "id": "131356111",
        "index": "sku",
        "path": "brand_id",
        "routing": "brand_id"
      }
    }
  }
}
```

##### [Term](https://opensearch.org/docs/latest/query-dsl/term/term/)
```rust
let term = Term::new("arcticle", "43935055")
    .case_insensitive(true)
    .boost(0.6);

let query = Query::new().query(term);
```

Generates the following request:
```json
{
  "query": {
    "term": {
      "arcticle": {
        "boost":0.6,
        "case_insensitive":true,
        "value":"43935055"
      }
    }
  }
}
```

#### [Compound-query](https://opensearch.org/docs/latest/query-dsl/compound/index/)

##### [Boolean](https://opensearch.org/docs/latest/query-dsl/compound/bool/)

###### Simple query 
```rust
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![CompoundQueryBoolean::Match(match_value)]);
    let query = Query::new().query(boolean);
```

Generates the following request
```json
{
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
    }
```

###### Query with nested bool-query
```rust
    let term = Term::new("name", "Деталь");
    let must_boolean = Bool::new().must_not(vec![term]);

    let finish_boolean = Bool::new().should(vec![must_boolean]);
    let query = Query::new().query(finish_boolean);
```

Generates the following request
```json
{
  "query": {
    "bool": {
      "should": [
        {
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
      ]
    }
  }
}
```

##### [Boosting](https://opensearch.org/docs/latest/query-dsl/compound/boosting/)
```rust
let match_positive = Match::new()
        .field("product_id")
        .value("46118216");
let match_negative = Match::new()
        .field("name")
        .value("Деталь");
let boosting = Boosting::new(match_positive, match_negative).negative_boost(0.1f64);

let query = Query::new().query(boosting);
```

Generates the following request

```json
{
  "query": {
    "boosting": {
      "positive": {
        "match": {
          "product_id": {
            "query": "46118216"
          }
        }
      },
      "negative": {
        "match": {
          "name": {
            "query": "Деталь"
          }
        }
      },
      "negative_boost": 0.1
    }
  }
}
```

##### [Constant score](https://opensearch.org/docs/latest/query-dsl/compound/constant-score/)
```rust
let match_value = Match::new().field("brand").value("Инструменты СТО");
let constant_score = ConstantScore::new(match_value);
let query = Query::new().query(constant_score);
```

Generates the following request

```json
{
  "query": {
    "constant_score": {
      "filter": {
        "match": {
          "brand": {
            "query":"Инструменты СТО"
          }
        }
      }
    }
  }
}
```

##### [Disjunction Max](https://opensearch.org/docs/latest/query-dsl/compound/disjunction-max/)
```rust
let match_positive = Match::new()
    .field("name")
    .value("Автозапчасть");
let match_negative = Match::new()
    .field("product_id")
    .value("46118319");
let term = Term::new("catalogue_id", 13558);
let dis_max = DisMax::new(vec![QueryField::Match(match_positive), QueryField::Match(match_negative), QueryField::Term(term)]);
let query = Query::new().query(dis_max);
```

Generates the following request

```json
{
  "query": {
    "dis_max": {
      "queries": [
        { "match": { "name": { "query": "Автозапчасть" }}},
        { "match": { "product_id":  { "query": "46118319" }}},
        { "term": {"catalogue_id": { "value": 13558 }}}
      ]
    }
  }
}
```

### Development plans
- Full-text queries (https://opensearch.org/docs/latest/query-dsl/full-text/index/)
  - Intervals(https://opensearch.org/docs/latest/query-dsl/full-text/intervals/)
- Compound queries (https://opensearch.org/docs/latest/query-dsl/compound/index/)
  - Function score (https://opensearch.org/docs/latest/query-dsl/compound/function-score/)
  - Hybrid (https://opensearch.org/docs/latest/query-dsl/compound/hybrid/)
- Aggregations (https://opensearch.org/docs/latest/aggregations/index/)
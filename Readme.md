# os-query-builder-rs
[![Version](https://img.shields.io/crates/v/os-query-builder-rs)](https://crates.io/crates/os-query-builder-rs)
[![License](https://img.shields.io/crates/l/os-query-builder-rs)](License)

Библиотека для формирования запросов для Open Search.

- [Установка](#установка)
- [Примеры использования](#примеры-использования)
  - [MultiMatch](#multimatch)
  - [Term-level](#term-level)
    - [Terms-query](#terms-query)
    - [Terms-lookup](#terms-lookup) 
    - [Term](#term)
  - [Compound-query](#compound-query)
    - [Boolean](#boolean) 
- [Планы развития](#планы-развития)

## Установка
```toml
[dependencies]
os-query-builder-rs = "0.1.9"
```

### Примеры использования

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

Сформирует следующий запрос

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

Сформирует следующий запрос:
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

Сформирует следующий запрос:
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

Сформирует следующий запрос:
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

###### Простой запрос 
```rust
    let match_value = Match::new().field("brand").value("FIAT");
    let boolean = Bool::new().must(vec![CompoundQueryBoolean::Match(match_value)]);
    let query = Query::new().query(boolean);
```

Сформирует следующий запрос
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

###### Запрос с вложенным bool-query
```rust
    let term = Term::new("name", "Деталь");
    let must_boolean = Bool::new().must_not(vec![term]);

    let finish_boolean = Bool::new().should(vec![must_boolean]);
    let query = Query::new().query(finish_boolean);
```

Сформирует следующий запрос
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


### Планы развития
- Compound queries
  - Boosting (https://opensearch.org/docs/latest/query-dsl/compound/boosting/)
  - Constant score (https://opensearch.org/docs/latest/query-dsl/compound/constant-score/)
  - Disjunction max (https://opensearch.org/docs/latest/query-dsl/compound/disjunction-max/)
  - Function score (https://opensearch.org/docs/latest/query-dsl/compound/function-score/)
  - Hybrid (https://opensearch.org/docs/latest/query-dsl/compound/hybrid/)
- Aggregations (https://opensearch.org/docs/latest/aggregations/index/)
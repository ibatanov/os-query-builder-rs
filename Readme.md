# os-query-builder-rs
[![Version](https://img.shields.io/crates/v/os-query-builder-rs)](https://crates.io/crates/os-query-builder-rs)
[![License](https://img.shields.io/crates/l/os-query-builder-rs)](License)

Библиотека для формирования запросов для Open Search.

## Установка
`os-query-builder-rs = "0.1.3`

### Пример использования
```rust
let multi_match = MultiMatchQuery::new()
            .fields(vec!["brands", "articles"])
            .value("oc47")
            .operator(Operator::And)
            .search_type(Type::BestFields)
            .boost(2)
            .minimum_should_match(2u64);

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
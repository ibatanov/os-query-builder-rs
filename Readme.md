# os-query-builder-rs
[![Version](https://img.shields.io/crates/v/os-query-builder-rs)](https://crates.io/crates/os-query-builder-rs)
[![License](https://img.shields.io/crates/l/os-query-builder-rs)](License)

Library for generating queries for Open Search.

- [Installation](#installation)
- [Usage examples](#usage-examples)
- [Development plans](#development-plans)

## Installation
```toml
[dependencies]
os-query-builder-rs = "0.2.0"
```

### Usage examples

You can see usage examples in the tests

- Term-level queries
  - [Exists](tests/exists_test.rs)
  - [Fuzzy](tests/fuzzy_test.rs)
  - [IDs](tests/ids_test.rs)
  - [Prefix](tests/prefix_test.rs)
  - [Range](tests/range_test.rs)
  - [Regexp](tests/regexp_test.rs)
  - [Term](tests/term_test.rs)
  - [Terms](tests/terms_test.rs)
  - [Terms set](tests/terms_set_test.rs)
  - [Wildcard](tests/wildcard_test.rs)
- Full-text queries
  - [Match](tests/match_test.rs)
  - [Match Boolean Prefix](tests/match_boolean_prefix_test.rs)
  - [Match phrase](tests/match_phrase_test.rs)
  - [Match phrase prefix](tests/match_phrase_prefix_test.rs)
  - [Multi Match](tests/multi_match_test.rs)
  - [Query string](tests/query_string_test.rs)
  - [Simple query string](tests/simple_query_string_test.rs)
  - [Intervals](tests/intervals_test.rs) and [interval rules](tests/intervals_rule_test.rs)
- Compound query
  - [Boolean](tests/compound_query_boolean_test.rs)
  - [Boosting](tests/compound_query_boosting_test.rs)
  - [Constant score](tests/compound_query_constant_score.rs)
  - [Disjunction max](tests/compound_query_disjunction_max_test.rs)

### Development plans
- Compound queries (https://opensearch.org/docs/latest/query-dsl/compound/index/)
  - Function score (https://opensearch.org/docs/latest/query-dsl/compound/function-score/)
  - Hybrid (https://opensearch.org/docs/latest/query-dsl/compound/hybrid/)
- Aggregations (https://opensearch.org/docs/latest/aggregations/index/)

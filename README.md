# Drupal GraphQL benchmarks

Compares Drupal GraphQL module vs JSON:API module vs a simple Rust GraphQL server.

## Installation

1. Install Rust
2. Install DDEV

## Prepare servers for benchmark

Start Drupal
```
cd drupal
ddev start
```
Start Rust GraphQL server
```
cd rust_graphql
cargo run --bin graphql_server
```

## Run benchmark
```
cd rust_graphql
cargo bench
```

## Benchmark results
### Drupal without page_cache and dynamic_page_cache
Uncached scenario: GrapthQL and JSON:API perform almost the same for a simple use case (fetch 1 node).
```
test graphql_drupal ... bench:  49,489,875 ns/iter (+/- 7,584,876)
test graphql_rust   ... bench:   2,131,631 ns/iter (+/- 169,069)
test html_drupal    ... bench:  63,370,745 ns/iter (+/- 7,781,316)
test jsonapi_drupal ... bench:  49,536,670 ns/iter (+/- 5,061,998)
```

### Drupal with dynamic_page_cache only
```
test graphql_drupal ... bench:  50,276,749 ns/iter (+/- 7,006,028)
test graphql_rust   ... bench:   2,146,732 ns/iter (+/- 328,681)
test html_drupal    ... bench:  35,906,957 ns/iter (+/- 4,415,295)
test jsonapi_drupal ... bench:  36,569,905 ns/iter (+/- 3,429,808)
```

### Drupal with page_cache
```
test graphql_drupal ... bench:  49,998,725 ns/iter (+/- 8,333,911)
test graphql_rust   ... bench:   2,137,049 ns/iter (+/- 353,134)
test html_drupal    ... bench:   7,662,672 ns/iter (+/- 5,869,703)
test jsonapi_drupal ... bench:   7,352,740 ns/iter (+/- 6,444,475)
```

## Blackfire testing
Test call for profiling
```
blackfire curl -X POST --data '{ "query": "{  article(id: 1) {    id    title    author  } }" }'  https://drupal-graphql.ddev.site/graphql_example
```
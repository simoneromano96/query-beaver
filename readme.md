# Query Beaver

## A Rust database agnostic query builder

Inspired by:

* Knex.js

* Laravel query builder

* TQL

* pinto

The library first objective is to generate queries that can be then executed by a client.

What is the objective:

```rust

let query_builder = QueryBeaver::builder()
    .dialect(Dialects::Mysql)
    .build();

let query = query_builder
    .select(&['col1', 'col2'])
    .from('table_name')
    .to_query();

assert_eq!("select `col1`, `col2` from `table_name`", query);

```

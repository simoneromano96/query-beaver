extern crate query_beaver;

use query_beaver::dialects::Dialects;
use query_beaver::query::statement;

fn main() {
    let dialect = Dialects::Mysql;
    let statement = statement::Statement {
        dialect,
        clauses: vec![],
        table_name: "".to_string()
    };

    println!("{:?}", statement);
}
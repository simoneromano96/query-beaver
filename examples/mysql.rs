extern crate query_beaver;

use query_beaver::dialects::Dialects;
use query_beaver::query::statement;

fn main() {
    let dialect = Dialects::Mysql;

    println!("{:?}", statement);
}
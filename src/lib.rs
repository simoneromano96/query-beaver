#![forbid(unsafe_code)]

pub mod dialects;
pub mod query;

#[cfg(test)]
mod tests {
    use crate::dialects::Dialects;
    use crate::query::clause::ClauseBuilder;
    use crate::query::statement::StatementBuilder;

    #[test]
    fn demo_statement() {
        let statement1 = StatementBuilder::new()
            .with_dialect(Dialects::Mysql)
            .with_schema_or_db(String::from("db_name"))
            .build();

        println!("{:?}", statement1);

        let clause1 = ClauseBuilder::select(String::from("table"), None).build();

        println!("{:?}", clause1);

        let clause2 = ClauseBuilder::select(
            String::from("table"),
            Some(vec![String::from("a"), String::from("b")]),
        )
        .build();

        println!("{:?}", clause2);

        let statement2 = StatementBuilder::new()
            .with_dialect(Dialects::Mysql)
            .with_schema_or_db(String::from("db_name"))
            .push_clause(clause1)
            .push_clause(clause2)
            .build();

        println!("{:?}", statement2);
    }
}

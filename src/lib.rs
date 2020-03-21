#![forbid(unsafe_code)]
#[macro_use]
extern crate typed_builder;

pub mod dialects;
pub mod query;

#[cfg(test)]
mod tests {
    use crate::dialects::Dialects;
    use crate::query::clause::ClauseBuilder;
    use crate::query::statement::Statement;

    #[test]
    fn demo_statement() {
        // let statement1 = StatementBuilder::new()
        //     .with_dialect(Dialects::Mysql)
        //     .with_schema_or_db(String::from("db_name"))
        //     .build();
        let clause1 = ClauseBuilder::select(String::from("table"), None).build();

        let statement1: Statement = Statement::builder()
            .dialect(Dialects::Mysql)
            .schema_or_db("db_name")
            .clauses(vec![clause1])
            .build();

        println!("{:?}", statement1);

        let clause2 = ClauseBuilder::select(
            String::from("table"),
            Some(vec![String::from("a"), String::from("b")]),
        )
        .build();

        let statement2: Statement = Statement::builder()
            .dialect(Dialects::Mysql)
            .clauses(vec![clause2])
            .build();

        println!("{:?}", statement2);

        /*
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
        */
    }
}

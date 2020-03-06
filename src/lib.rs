#![forbid(unsafe_code)]

pub mod query;
pub mod dialects;

#[cfg(test)]
mod tests {
    use crate::query::statement::Statement;
    use crate::dialects::Dialects;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_should_be() {
        let table_name = "table";
        let mut statement = Statement::new()
            .with_dialect(Dialects::Mysql)
            .with_table(String::from("table")).build();

        println!("{:?}", statement);
    }
}

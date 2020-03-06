use crate::dialects::Dialects;
use crate::query::clause::Clause;

/// Statement implementation
#[derive(Copy, Clone, Debug)]
pub struct Statement {
    /// Specify dialect for conversion
    dialect: Dialects,
    /// Array of clauses completing a Statement
    clauses: Vec<Clause>,
    /// Table name
    table: String,
    /// Schema or Db name
    schema: Option<String>
}

impl Statement {
    /// Default statement
    pub fn new() -> Statement {
        Statement {
            dialect: Dialects::Mysql,
            clauses: vec![],
            table: "".to_string(),
            schema: None
        }
    }
    /// Set the dialect
    pub fn with_dialect(&mut self, dialect: Dialects) -> &mut Statement {
        self.dialect = dialect;
        self
    }
    /// Set the table name
    pub fn with_table(&mut self, table: String) -> &mut Statement {
        self.table = table;
        self
    }
    /// Build statement
    pub fn build(self) -> Statement {
        Statement {
            clauses: self.clauses,
            dialect: self.dialect,
            table: self.table,
            schema: self.schema
        }
    }
}

/*
pub trait StatementTrait {

}
*/
use crate::dialects::Dialects;
use crate::query::clause::Clause;

/// Statement implementation
#[derive(Debug)]
pub struct Statement {
    /// Array of clauses completing a Statement
    clauses: Vec<Clause>,
    /// Specify dialect for conversion
    dialect: Dialects,
    /// Schema or Db name
    schema_or_db: Option<String>
}

/*
pub trait StatementToQuery {
    fn to_query_string(&self: Statement) -> String {}
}
*/

/// Statement builder implementation
#[derive(Debug)]
pub struct StatementBuilder {
    /// Array of clauses completing a Statement
    clauses: Vec<Clause>,
    /// Specify dialect for conversion
    dialect: Dialects,
    /// Schema or Db name
    schema_or_db: Option<String>
}

impl StatementBuilder {
    /// Default statement
    pub fn new() -> StatementBuilder {
        StatementBuilder {
            dialect: Dialects::Mysql,
            clauses: vec![],
            schema_or_db: None
        }
    }
    /// Add a clause
    pub fn push_clause(&mut self, clause: Clause) -> &mut StatementBuilder {
        self.clauses.push(clause);
        self
    }
    /// Set the dialect
    pub fn with_dialect(&mut self, dialect: Dialects) -> &mut StatementBuilder {
        self.dialect = dialect;
        self
    }
    /// Set the schema or db name
    pub fn with_schema_or_db(&mut self, schema_or_db: String) -> &mut StatementBuilder {
        self.schema_or_db = Some(schema_or_db);
        self
    }
    /// Build statement
    pub fn build(self) -> Statement {
        Statement {
            clauses: self.clauses,
            dialect: self.dialect,
            schema_or_db: self.schema_or_db
        }
    }
}

/*
pub trait StatementTrait {

}
*/
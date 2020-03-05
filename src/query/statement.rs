use crate::dialects::Dialects;

#[derive(Debug)]
pub struct Expression {}

/// Clause implementation
#[derive(Debug)]
pub struct Clause {
    expression: Option<Expression>,
}

/// Statement implementation
#[derive(Debug)]
pub struct Statement {
    /// Specify dialect for conversion
    pub dialect: Dialects,
    /// Array of clauses completing a Statement
    pub clauses: Vec<Clause>,
    /// Table name
    pub table_name: String,
}

pub trait StatementTrait {

}

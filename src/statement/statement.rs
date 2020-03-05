use crate::dialects::Dialects;

pub struct Expression {}

/// Clause implementation
pub struct Clause {
    expression: Option<Expression>,
}

/// Statement implementation
pub struct Statement {
    /// Specify dialect for conversion
    dialect: Dialects,
    /// Array of clauses completing a Statement
    clauses: Vec<Clause>,
    /// Table name
    table_name: String,
}

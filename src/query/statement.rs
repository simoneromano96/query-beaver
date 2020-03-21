use crate::dialects::Dialects;
use crate::query::clause::Clause;

/// Statement implementation
#[derive(Debug, TypedBuilder)]
pub struct Statement<'a> {
    /// Array of clauses completing a Statement
    clauses: Vec<Clause>,
    /// Specify dialect for conversion
    dialect: Dialects,
    /// Schema or Db name
    #[builder(default=None, setter(strip_option))]
    schema_or_db: Option<&'a str>,
}

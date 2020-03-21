/*
#[derive(Debug)]
pub struct Expression {}

#[derive(Debug)]
pub struct Predicate {}
*/

/// Clause implementation
#[derive(Debug)]
pub struct Clause {
    /// Table name
    table: String,
    //expression: Option<Expression>,
    //predicate: Option<Predicate>,
    expression: String,
    // predicate: Option<String>,
}

/// ClauseBuilder implementation
#[derive(Debug)]
pub struct ClauseBuilder {
    /// Table name
    table: String,
    //expression: Option<Expression>,
    //predicate: Option<Predicate>,
    expression: String,
    // predicate: Option<String>,
}

impl ClauseBuilder {
    /// Default clause
    pub fn new() -> ClauseBuilder {
        ClauseBuilder {
            table: String::from(""),
            expression: String::from(""),
            // predicate: None
        }
    }
    /// Select shortcut
    pub fn select(table: String, columns: Option<Vec<String>>) -> ClauseBuilder {
        let mut expression = String::from("select");
        for column in columns.unwrap_or(vec![String::from("*")]) {
            expression = format!("{} {}", expression, column);
        }
        expression = format!("{} from {}", expression, table);
        ClauseBuilder {
            table,
            expression,
            // predicate: None
        }
    }
    /// Build clause
    pub fn build(self) -> Clause {
        Clause {
            table: self.table,
            expression: self.expression,
        }
    }
}

/*
pub trait  {

}
*/

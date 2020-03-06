#[derive(Debug)]
pub struct Expression {}

#[derive(Debug)]
pub struct Predicate {}

/// Clause implementation
#[derive(Debug)]
pub struct Clause {
    expression: Option<Expression>,
    predicate: Option<Predicate>
}

/*
pub trait  {

}
*/
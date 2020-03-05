#![forbid(unsafe_code)]
pub mod query;
pub mod dialects;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

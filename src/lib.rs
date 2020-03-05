#![forbid(unsafe_code)]
mod statement;
mod dialects;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

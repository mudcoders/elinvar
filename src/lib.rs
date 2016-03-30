pub mod rng;
pub mod net;

/// Simple example of a function that returns a string
///
/// # Example
///
/// ```
/// use libelinvar::hello;
/// hello();
/// ```
pub fn hello() -> String {
    return "Elinvar".to_owned();
}

#[cfg(test)]
mod test {
    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!("Elinvar", hello());
    }
}

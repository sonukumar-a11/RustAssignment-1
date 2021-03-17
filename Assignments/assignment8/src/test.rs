#[cfg(test)]
mod tests {
    use crate::error_handler::check_handling::even_odd;

    #[test]
    fn check_even() {
        assert_eq!(even_odd(25), "Please put right integer".to_string());
    }

    #[test]
    fn check_even_next() {
        assert_eq!(even_odd(0), "Even");
    }

    #[test]
    fn error_check() {
        assert_eq!(even_odd(3), "Please put right integer".to_string());
    }
}

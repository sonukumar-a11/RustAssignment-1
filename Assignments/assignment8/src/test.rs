#[cfg(test)]
#[test]
fn check_even() {
    use crate::error_handling::_even_odd;
    assert_eq!(_even_odd(25), "Please put right way".to_string());

    assert_eq!(_even_odd(34), "Even");

    assert_eq!(_even_odd(0), "Even");

    assert_eq!(_even_odd(3), "Please put right way".to_string());
}

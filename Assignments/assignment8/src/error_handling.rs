/// This function take input to as integer
///
/// #Arguments
///
///check_number: take input from user
///
/// #Return
///
/// Returns the String to give the detect  output....
pub fn _even_odd(check_number: i32) -> String {
    let result = _give_even_odd(check_number);

    match result {
        Ok(result) => result,

        Err(_error) => "Please put right way".to_string(),
    }
}

/// This function take input to as
///
/// #Arguments
///
/// check_number: to check the number Condition
///
/// #Return
///
/// Returns the String even and odd
fn _give_even_odd(check_number: i32) -> Result<String, String> {
    if check_number ^ 1 == check_number + 1 {
        Ok("Even".to_string())
    } else {
        Err("Odd".to_string())
    }
}

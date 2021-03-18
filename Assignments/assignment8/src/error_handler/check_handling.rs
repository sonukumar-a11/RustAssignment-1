/// even_odd which handle the response of calling function
///
/// #Arguments
///
///check_number: A check_number is {Integer} object whose use to manipulate
///
/// #Return
///
/// Returns String which  maintain and handle the function output....
pub fn even_odd(check_number: i32) -> String {
    let result = give_even_odd(check_number);

    match result {
        Ok(result) => result,

        Err(_) => "Please put right integer".to_string(),
    }
}

/// give_even_odd which used check the number is even or odd
///
/// #Arguments
///
///check_number: A check_number is {Integer} object whose use to check Even or odd
///
/// #Return
///
/// Returns Result enum which used to handle Error and value both
fn give_even_odd(check_number: i32) -> Result<String, String> {
    if check_number ^ 1 == check_number + 1 {
        Ok("Even".to_string())
    } else {
        Err("Odd".to_string())
    }
}

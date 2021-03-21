/// return_palindrome which is used to check whether a palindrome or not
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns boolean  which maintains  check whether a palindrome or not
fn return_palindrome(iterable: Vec<i32>, eiter_start: usize, eiter_end: usize) -> bool {
    if eiter_start == eiter_end {
        return true;
    }

    if iterable.get(eiter_start) != iterable.get(eiter_end) {
        return false;
    }

    if eiter_start < eiter_end + 1 {
        return return_palindrome(iterable, eiter_start + 1, eiter_end - 1);
    }
    true
}

/// check_palindrome which is used to check whether a palindrome or not
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns Result<bool,String>  which maintains check whether a palindrome or not and error as well
pub fn check_palindrome(iterable: Vec<i32>) -> Result<bool, String> {
    if iterable.is_empty() {
        return Err("Iterable is not valid".to_string());
    }

    let eiter_start: usize = 0;
    let eiter_end: usize = iterable.len() - 1;
    let output: bool = return_palindrome(iterable, eiter_start, eiter_end);

    log::info!("check whether a palindrome or not");

    Ok(output)
}

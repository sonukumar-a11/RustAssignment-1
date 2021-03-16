/// return_palindrome function check the vector is palindrome or not
///
/// #Arguments
///
///
///iterable - take a iterable as vector in which  manipulation feasible
///eiter_start : - starting value to check
///eiter_end :- End value to check
///
/// #Return
///
/// Returns bool value after check the vector is palindrome or not....
fn _return_palindrome(iterable: Vec<i32>, eiter_start: usize, eiter_end: usize) -> bool {
    if eiter_start == eiter_end {
        return true;
    }

    if iterable.get(eiter_start) != iterable.get(eiter_end) {
        return false;
    }

    if eiter_start < eiter_end + 1 {
        return _return_palindrome(iterable, eiter_start + 1, eiter_end - 1);
    }

    true
}
/// check_palindrome function check the vector is palindrome or not
///
/// #Arguments
///
///iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// Returns bool value after check the vector is palindrome or not....
pub fn _check_palindrome(iterable: Vec<i32>) -> bool {
    let eiter_start: usize = 0;
    let eiter_end: usize = iterable.len() - 1;
    let output: bool = _return_palindrome(iterable, eiter_start, eiter_end);

    log::info!("List : {}", output);
    output
}

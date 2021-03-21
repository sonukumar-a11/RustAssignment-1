/// reverse_recursively which used as a helper function to find the reverse the vector recursively
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
fn reverse_recursively(iterable: &mut Vec<i32>, either_start: usize, either_end: usize) {
    if either_start >= either_end {
        return;
    }
    iterable.swap(either_start, either_end);
    reverse_recursively(iterable, either_start + 1, either_end - 1);
}

/// reverse which reverse the vector of integer
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// return Result<Vec<i3@>,String> maintaining  the  reverse of vector and handle error as well
pub fn reverse(iterable: &mut Vec<i32>) -> Result<Vec<i32>, String> {
    if iterable.is_empty() {
        return Err("Iterable is not valid".to_string());
    }
    let eiter_start: usize = 0;
    let eiter_end: usize = iterable.len() - 1;
    reverse_recursively(iterable, eiter_start, eiter_end);
    let complete_rev_iterable = iterable.clone();

    log::info!("List : {:?}", complete_rev_iterable);

    Ok(complete_rev_iterable)
}

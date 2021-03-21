/// duplicate_element which is used to append the duplicate element
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of {Integer}
///
/// #Return
///
/// Returns Vector<T> maintain  the duplicate of each element as well as handle error
pub fn duplicate_element(iterable: &mut Vec<i32>) -> Vec<i32> {
    if iterable.is_empty() {
        panic!("Why put iterable empty");
    }

    let size = iterable.len();

    for item in 0..size {
        let value = iterable[item];
        iterable.push(value);
        iterable.push(value);
    }
    iterable[size..].to_vec()
}

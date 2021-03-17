/// duplicate_element which add the duplicate element
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns Vector maintain  the duplicate of each element
pub fn duplicate_element(iterable: &mut Vec<i32>) -> Vec<i32> {
    let size = iterable.len();

    for item in 0..size {
        let value = iterable[item];
        println!("{}", value);
        iterable.push(value);
        iterable.push(value);
    }
    iterable[size..].to_vec()
}

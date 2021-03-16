/// _duplicate_element function return the duplicate element with filling
///
/// #Arguments
///
///iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// Returns Vector value to give the duplicate value
pub fn _duplicate_element(iterable: &mut Vec<i32>) -> Vec<i32> {
    let size = iterable.len();

    for item in 0..size {
        let value = iterable[item];
        println!("{}", value);
        iterable.push(value);
        iterable.push(value);
    }
    iterable[size..].to_vec()
}

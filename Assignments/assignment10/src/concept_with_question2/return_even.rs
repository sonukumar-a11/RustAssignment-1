/// first_even which is used to find out the first even number
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns Result<i32,String>> value as first even number and handle the Error as well as ...........
pub fn first_even(iterable: Vec<i32>) -> Result<i32, String> {
    if iterable.is_empty() {
        return Err("Iterable is not valid".to_string());
    }
    let mut index = 0;
    loop {
        if iterable[index] & 1 == 0 {
            return Ok(iterable[index]);
        }
        index += 1;
    }
}

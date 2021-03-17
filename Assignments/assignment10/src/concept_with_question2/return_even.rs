/// first_even which find out the first even number
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns { Integer } value as first even number
pub fn first_even(iterable: Vec<i32>) -> i32 {
    let mut index = 0;
    loop {
        if iterable[index] & 1 == 0 {
            return iterable[index];
        }
        index += 1;
    }
}

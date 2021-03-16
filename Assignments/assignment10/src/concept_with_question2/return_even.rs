/// _first_even function return the first even value present in the iterable
///
/// #Arguments
///
///iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// Returns i32 as  the first even value present in the iterable
pub fn _first_even(iterable: Vec<i32>) -> i32 {
    let mut index = 0;
    let mut _value = -1;
    loop {
        if iterable[index] & 1 == 0 {
            _value = iterable[index];
            break;
        }
        index += 1;
    }
    _value
}

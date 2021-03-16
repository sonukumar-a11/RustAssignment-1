/// reverse_something  give the reverse of vector
///
/// #Arguments
///
/// iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// change the given vector to reverse through recursion
fn _reverse_something(iterable: &mut Vec<i32>, either_start: usize, either_end: usize) {
    if either_start >= either_end {
        return;
    }
    iterable.swap(either_start, either_end);
    _reverse_something(iterable, either_start + 1, either_end - 1);
}
/// _check_reverse  give the reverse of vector
///
/// #Arguments
///
/// iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// return the  vector having store reverse of given vector
pub fn _check_reverse(iterable: &mut Vec<i32>) -> Vec<i32> {
    let eiter_start: usize = 0;
    let eiter_end: usize = iterable.len() - 1;
    _reverse_something(iterable, eiter_start, eiter_end);
    let clone_iterable = iterable.clone();

    log::info!("List : {:?}", clone_iterable);

    clone_iterable
}

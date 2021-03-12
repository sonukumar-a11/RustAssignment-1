/// sort_anything function take in generic way and sort any type
///
/// #Arguments
///
///iterable: take input from user
///
/// #Return
///
/// Return  the sorted way....
pub fn _sort_anything<T: std::cmp::PartialOrd>(iterable: &mut [T]) -> &mut [T] {
    for index1 in 0..iterable.len() {
        let mut small = index1;
        for index2 in (index1 + 1)..iterable.len() {
            if iterable[index2] < iterable[small] {
                small = index2;
            }
        }
        iterable.swap(small, index1);
    }
    iterable
}

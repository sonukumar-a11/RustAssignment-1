/// sort_elements which sort the any type element
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of generic type data
///
/// #Return
///
/// Returns array maintain sorted value
pub fn sort_elements<T: std::cmp::PartialOrd>(iterable: &mut [T]) -> &mut [T] {
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

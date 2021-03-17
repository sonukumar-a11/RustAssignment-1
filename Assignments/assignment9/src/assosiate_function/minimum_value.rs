/// min_value take parameter in generic way and used to find the minimum
///
/// #Arguments
///
/// piece1: A piece1 is T type which contains the any value of generic type data
/// piece2: A piece2 is T type which contains the any value of generic type data
///
/// #Return
///
/// Return T type Having value of minimum between them
pub fn min_value<T: std::cmp::PartialOrd>(piece1: T, piece2: T) -> T {
    if piece1 < piece2 {
        piece1
    } else {
        piece2
    }
}

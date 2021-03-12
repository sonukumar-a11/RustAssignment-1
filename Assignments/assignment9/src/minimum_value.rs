/// min_value function take parameter in generic way and used to find the minimum
///
/// #Arguments
///
///piece1: take generic from user
/// piece2:- take generic value
///
/// #Return
///
/// Return  minimum value between them....
pub fn _min_value<T: std::cmp::PartialOrd>(piece1: T, piece2: T) -> T {
    if piece1 < piece2 {
        piece1
    } else {
        piece2
    }
}

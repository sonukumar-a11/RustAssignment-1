use ::log::*;

/// min_value take parameter in generic way and used to find the minimum value
///
/// #Arguments
///
/// piece1: A piece1 is T type object which contains the any Primitive value...
/// piece2: A piece2 is T type object  which contains the any Primitive value...
///
/// #Return
///
/// Return Result<T,E>  type Having value of minimum between them and handle error as well.....
pub fn min_value<T: std::cmp::PartialOrd>(piece1: T, piece2: T) -> Result<T, String> {
    if piece1 == piece2 {
        error!("Calling function not needed");
        Err("Given piece are input / invalid input".to_string())
    } else if piece1 < piece2 {
        Ok(piece1)
    } else {
        Ok(piece2)
    }
}

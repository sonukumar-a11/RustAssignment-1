use log::*;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Value};

/// nth function finds the nth element from the iterator.
///
/// #Arguments
///
/// 'iterable'- A iterable enum object which contains the list of numbers.
///
/// 'position'- Store the find the value of position
///
/// #Return
///
/// Return the Result<T,E>  T consider number contains nth element an handle error as well as.....
pub fn nth(position: i32, iterable: Store) -> Result<i32, String> {
    if iterable == Nil {
        error!("Empty Box Provided");
        return Err("Please provide valid Input".to_string());
    }
    let result = recursion(position - 1, iterable, 0);
    Ok(result)
}

/// nth function finds the nth element from the iterator.
///
/// #Arguments
///
/// 'iterable'- A iterable enum object which contains the list of numbers.
///
/// 'position'- Store the find the value of position
///
/// 'counter' used to iterate and update in very iteration
/// #Return
///
/// Return the i32 number contains nth element
pub fn recursion(position: i32, iterator: Store, counter: i32) -> i32 {
    info!("finds number at counter");
    match iterator {
        Nil => -1,
        Value(current, _) if counter == position => current,
        Value(_, iterator) => recursion(position, *iterator, counter + 1),
    }
}

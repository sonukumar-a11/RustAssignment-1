use log::*;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Value};

/// first_repeated function search the first repeated element
///
/// #Arguments
///
/// 'Store'- A iterable of enum object which contains the list of numbers.
///
/// #Return
///
/// Return the Result<T,E>  T consider number contains first_repeated_element an handle error as well as.....
pub fn first_repeated(iterable: Store) -> Result<i32, String> {
    if iterable == Nil {
        error!("Empty Box Provided");
        return Err("Please provide valid Input".to_string());
    }
    let result = consecutive(iterable, -1);
    Ok(result)
}

/// consecutive function use to match list object and find first repeated number through recursion.
///
/// #Arguments
///
/// 'previous_one' - An i32 variable containing the previous value in Cons tuple of List enum. Take default -1
///
/// 'iterable'- A iterable enum object which contains the list.
///
/// #Return
///
/// Return the i32 number containing first repeated number.
fn consecutive(iterable: Store, previous_one: i32) -> i32 {
    info!("finds number at previous_one");
    match iterable {
        Nil => -1,

        Value(initial, _iterable) if initial == previous_one => initial,

        Value(initial, iterable) => consecutive(*iterable, initial),
    }
}

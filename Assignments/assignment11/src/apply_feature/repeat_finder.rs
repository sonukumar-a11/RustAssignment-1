use log::info;

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
/// Return the i32 number containing first repeated element
pub fn first_repeated(iterable: Store) -> i32 {
    consecutive(iterable, -1)
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

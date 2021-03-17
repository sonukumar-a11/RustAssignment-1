use log::info;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Value};

/// third_odd function finds the third odd number.
///
/// #Arguments
///
/// 'iterable'- A iterable enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number contains the third odd number

pub fn third_odd(iterable: Store) -> i32 {
    find_odd(iterable, 3)
}

/// find_odd function finds the third odd number.
///
/// #Arguments
///
/// 'iterable'- A iterable enum object which contains the list of numbers.
///
/// 'iterator'-Used to find the odd number.
///
/// #Return
///
/// Return the i32 number contains the third odd number

pub fn find_odd(iterable: Store, iterator: i32) -> i32 {
    info!("finds number at {} odd_index", iterator);
    match iterable {
        Nil => -1,

        Value(initial, _iterable) if iterator == 1 && &initial & 1 == 1 => initial,

        Value(initial, iterable) if &initial & 1 == 1 => find_odd(*iterable, iterator - 1),

        Value(_initial, iterable) => find_odd(*iterable, iterator),
    }
}

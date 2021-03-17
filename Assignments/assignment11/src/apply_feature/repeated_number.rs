use log::info;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Value};

/// second_repeated function finds the second element repeated.
///
/// #Arguments
///
/// 'iterable'- A iterable enum object which contains the list of numbers.
///
/// #Return
///
/// Return the i32 number contains second repeated number.

pub fn second_repeated(iterable: Store) -> i32 {
    recursion(-1, iterable, 0)
}

/// recursion function use recursion to match list object and find second repeated number.
///
/// #Arguments
///
/// 'previous'- An i32 variable containing the previous value in Cons tuple of List enum.
///
/// 'iterable'- A List enum object which contains the list of numbers.
///
/// 'occurrence'- Store the count to find the second number
///
/// #Return
///
/// Return the i32 number contains second repeated number.
pub fn recursion(previous: i32, iterable: Store, occurrence: i32) -> i32 {
    info!("finds first repeated number");
    match iterable {
        Nil => -1,
        Value(initial, _) if initial == previous && occurrence == 1 => initial,
        Value(initial, iterable) if initial == previous => {
            recursion(initial, *iterable, occurrence + 1)
        }
        Value(initial, iterable) => recursion(initial, *iterable, occurrence),
    }
}
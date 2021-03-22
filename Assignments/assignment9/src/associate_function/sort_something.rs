use std::fmt::Display;

use log::*;

/// sort_elements which is used to sort the any type iterable
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of T type
///
/// #Return
///
/// Returns Option<T> maintain sorted value........
pub fn sort_elements<T: std::cmp::PartialOrd + Display + std::fmt::Debug>(
    iterable: &mut [T],
) -> Option<&mut [T]> {
    if iterable.is_empty() {
        error!("Given invalid input");
        panic!("Provided Empty iterable 'Bad Practice' ");
    }

    for index1 in 0..iterable.len() {
        let mut small = index1;
        for index2 in (index1 + 1)..iterable.len() {
            if iterable[index2] < iterable[small] {
                small = index2;
            }
        }
        iterable.swap(small, index1);
    }
    info!("Given iterable is valid :{:?}", iterable);
    Some(iterable)
}

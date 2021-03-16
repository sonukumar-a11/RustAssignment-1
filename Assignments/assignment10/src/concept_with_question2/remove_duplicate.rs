use std::collections::BTreeSet;
/// _remove_same function return unique value
///
/// #Arguments
///
///iterable - take a iterable as vector in which  manipulation feasible
///
/// #Return
///
/// Returns Vector value to give the unique vector
pub fn _return_unique(iterable: &mut Vec<i32>) -> Vec<i32> {
    let mut stare: BTreeSet<&i32> = BTreeSet::new();
    for item in iterable {
        stare.insert(item);
    }
    let mut array_val: Vec<i32> = Vec::new();
    for value in stare.iter() {
        array_val.push(**value);
    }
    array_val
}

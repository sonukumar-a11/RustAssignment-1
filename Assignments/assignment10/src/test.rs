#[cfg(test)]
#[test]
fn check_hash_sum() {
    use crate::hash::_sumcondtional;
    use std::collections::HashMap;
    let mut ourmap = HashMap::new();
    ourmap.insert("sonu", 15);
    ourmap.insert("aadarsh", 16);
    ourmap.insert("monu", 94);
    ourmap.insert("shubranu", 4);
    ourmap.insert("satyendra", 21);
    assert_eq!(_sumcondtional(ourmap, "nu"), 113);

    let mut usermap = HashMap::new();
    usermap.insert("Rekha", 65);
    usermap.insert("Reksh", 36);
    usermap.insert("djekfth", 14);
    usermap.insert("shubranu", 4);
    usermap.insert("satyendra", 21);
    assert_eq!(_sumcondtional(usermap, "ek"), 115);
}
#[test]
fn isolating() {
    use crate::concept_with_question2::palindrome_check::_check_palindrome;
    let output = vec![1, 1];
    assert_eq!(_check_palindrome(output), true);
    assert_eq!(_check_palindrome(vec![1, 2]), false);
}
#[test]
fn perverse() {
    use crate::concept_with_question2::rev_iterable::_check_reverse;
    let mut output = vec![1, 1];
    assert_eq!(_check_reverse(&mut output), [1, 1]);
    let mut output = vec![1, 3, 4];
    assert_eq!(_check_reverse(&mut output), [4, 3, 1]);
}
#[test]
fn check_drop() {
    use crate::concept_with_question2::nth_value::_Drop;
    let drop_iterable: _Drop = _Drop {
        nth_value: 3,
        iterable: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
    };
    assert_eq!(drop_iterable._drop_element(), [1, 2, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        _Drop {
            nth_value: 6,
            iterable: vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        }
        ._drop_element(),
        [1, 2, 3, 4, 5, 7, 8, 9]
    );
}
#[test]
fn summation_duplicate_check() {
    use crate::concept_with_question2::add_duplicate::_duplicate_element;
    let mut output = vec![1, 2];
    assert_eq!(_duplicate_element(&mut output), [1, 1, 2, 2]);
    let mut output = vec![1, 2, 3, 4];
    assert_eq!(_duplicate_element(&mut output), [1, 1, 2, 2, 3, 3, 4, 4]);
}

#[test]
fn check_even() {
    use crate::concept_with_question2::return_even::_first_even;
    let output = vec![1, 2];
    assert_eq!(_first_even(output), 2);
    let output = vec![1, 0];
    assert_eq!(_first_even(output), 0);
}

#[test]
fn check_duplicate_exist() {
    use crate::concept_with_question2::remove_duplicate::_return_unique;
    let mut output = vec![0, 0, 1, 1];
    assert_eq!(_return_unique(&mut output), [0, 1]);
    let mut output = vec![0, 0, 1, 1, 2, 2, 3, 4, 4, 5, 3];
    assert_eq!(_return_unique(&mut output), [0, 1, 2, 3, 4, 5]);
}

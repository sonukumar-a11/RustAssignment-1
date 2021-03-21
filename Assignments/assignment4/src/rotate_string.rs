use std::iter::FromIterator;
use std::iter::Iterator;

/// This function check if two strings are rotations of each other
///
/// #Arguments
///
/// check_rotating1:- Take a String input In sorted form
/// check_rotating2:-  Take a another input in sorted form
///
/// #Return
///
/// Return boolean value to check the both are rotation of each other
fn rotate(check_rotating1: String, check_rotating2: String, trackindex: usize) -> bool {
    if check_rotating1.len() != check_rotating1.len() {
        return false;
    }
    if check_rotating1.chars().nth(trackindex) != check_rotating2.chars().nth(trackindex) {
        return false;
    }
    if trackindex > check_rotating1.len() || trackindex > check_rotating2.len() {
        true
    } else {
        rotate(check_rotating1, check_rotating2, trackindex + 1)
    }
}

/// This function check the rotation exist in given input
///
/// #Arguments
///
/// input1:- take input 1 for check rotation
/// input2:- take input2 for rotation is exist or not
///
/// #Return
///
/// Return String value for unit testing
pub fn check_rotation(input1: &str, input2: &str) -> String {
    let sorted_input1 = sort_char(input1);
    let sorted_input2 = sort_char(input2);
    let isrotate1 = rotate(sorted_input1, sorted_input2, 1);
    let mut result = String::new();
    if isrotate1 {
        result.push_str("Rotate Exist");
    } else {
        result.push_str("Rotate Not Exist");
    }
    result
}

/// This function return sorted string using itertools crate
///
/// #Arguments
///
/// input1:- Sorted the input in character vise

/// #Return
///
/// Return String in sorted form
pub fn sort_char(input: &str) -> String {
    let mut s: Vec<char> = input.chars().collect();
    s.sort_unstable();

    String::from_iter(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rotation_exist() {
        let value = check_rotation("lkjh", "jklh");
        assert_eq!(None.unwrap_or_else(|| value), "Rotate Exist");
        assert_eq!(check_rotation("abcd", "cdba"), "Rotate Exist");
        assert_eq!(check_rotation("klnm", "bkfg"), "Rotate Not Exist");
        assert_eq!(check_rotation("klnm", "tswq"), "Rotate Not Exist");
    }
}

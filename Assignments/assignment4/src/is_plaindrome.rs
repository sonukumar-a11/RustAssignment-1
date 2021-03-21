///This function checks if the string is palindrome or not
///
/// #Arguments
///checkInput:-
///
///travel_from_start:-
///
/// move_form_last::-
///
/// Description:- Check move from  first and last also with middle substring with second if condition
///
/// #Return
///
///
/// Return boolean value to check given input is palindrome or not
fn isolating(checkinput: &str, travel_from_start: usize, move_from_last: usize) -> bool {
    if travel_from_start == move_from_last {
        return true;
    }

    if checkinput.as_bytes()[travel_from_start] != checkinput.as_bytes()[move_from_last] {
        return false;
    }

    if travel_from_start < move_from_last + 1 {
        return isolating(checkinput, travel_from_start + 1, move_from_last - 1);
    }
    true
}

//This function checks if the string is palindrome or not
///
/// #Arguments
///travel_input:-Take a input from main function to perform manipulation
///
///
/// Description:- call isplaindrome function form passing some default argument value
///
/// #Return
///
//
/// Return String value for unit testing
pub fn check_palindrome(travel_input: &str) -> String {
    let traval_input_length: usize = travel_input.len();
    let implanted = isolating(travel_input, 0, traval_input_length - 1);
    let mut result = String::new();
    if implanted  {
        result.push_str("Plaindrome");
    } else {
        result.push_str("Not Plaindrome");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_palindrome_sequence_0() {
        assert_eq!(check_palindrome("abba"), "Plaindrome");
        assert_eq!(check_palindrome("defghigjk"), "Not Plaindrome");
        assert_eq!(check_palindrome("Sonu"), "Not Plaindrome");
        assert_eq!(check_palindrome("kllk"), "Plaindrome");
    }
}

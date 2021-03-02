
///This function checks if the string is palindrome or not
fn isplaindrome(input: &str,s:usize,e:usize) -> bool {

    if s == e
    {
        return true;
    }
    // If first and last
    // characters do not match
    if input.as_bytes()[s] != input.as_bytes()[e]
    {
        return false;
    }
    // If there are more than
    // two characters, check if
    // middle substring is also
    // palindrome or not.
    if s < e + 1 {
        return isplaindrome(input ,  s + 1, e - 1);
    }
    return true;
}
pub fn check_palindrome(input: &str) -> String {
    let lenght: usize = input.len();
    let isplaindrome1 = isplaindrome(
        input,
        0,
        lenght - 1
    );
    let mut result = String::new();
    if isplaindrome1 == true
    {
        result.push_str("Palindrome");
    } else {
        result.push_str("Not Palindrome");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input_string() {
        assert_eq!(check_palindrome("abba"),"Plaindrome");
    }
}
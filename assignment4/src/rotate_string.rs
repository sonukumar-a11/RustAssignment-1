
use itertools::Itertools;

fn rotate(str1: String, str2: String, index:usize) ->bool
{
    if str1.len()!=str2.len()
    {
        return false;
    }
    if str1.as_bytes()[index] != str2.as_bytes()[index]
    {
        return false
    }
    if index > str1.len()  || index > str2.len()
    {
        return true
    }
    else
    {
        rotate(str1, str2, index + 1)
    }
}
pub fn check_rotation(input1: &str, input2 :&str) ->String {
    let str1: String = input1.chars().sorted().rev().collect::<String>();
    let str2: String = input2.chars().sorted().rev().collect::<String>();
    let isrotate1 = rotate(
        str1,
        str2,
        1
    );
    let mut result = String::new();
    if isrotate1 == true
    {
        result.push_str("Rotate Exist");
    } else {
        result.push_str("Rotate Not exist");
    }
    result
}
mod tests {
    use super::*;

    #[test]
    fn check_input_string() {
        assert_eq!(check_rotation("abcd", "cdba"), "Rotate Exist");
    }
}
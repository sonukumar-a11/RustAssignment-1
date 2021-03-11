/// This function containing all substring
///
/// #Arguments
///
/// input_iterable - Taking String as input and generating substring of input
///
/// #Return
///
/// Returns a vector<Sting> having all substring input
pub fn _generating_substring(input_iterable: String) -> Vec<String> {
    if input_iterable.is_empty() {
        return vec!["".to_string()];
    }
    let mut rest_str: Vec<String> = Vec::new();
    let mut substring: &str;
    for i in 0..input_iterable.len() {
        for j in i..input_iterable.len() {
            substring = &input_iterable[i..(j + 1)];
            rest_str.push(substring.to_string());
        }
    }
    rest_str
}

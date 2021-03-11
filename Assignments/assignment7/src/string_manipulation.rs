/// This function take input to as
///
/// #Arguments
///
///iterator1 - Take a first iterable as string
///iterator2 - Take a Second iterable as string
///iterator3 - Take a Third iterable as string
///
/// #Return
///
/// Returns the String to give the desired output....
pub fn _desideratum_output(iterator1: &str, iterator2: &str, iterator3: &str) -> String {
    let mut position = 0;
    let mut i: usize = 0;
    let mut array: Vec<char> = Vec::new();

    while i < iterator2.len() {
        if position % 2 == 0 {
            let compare_result = if iterator1.chars().nth(i) < iterator2.chars().nth(i) {
                iterator1.chars().nth(i)
            } else {
                iterator2.chars().nth(i)
            };
            let result_char = if compare_result < iterator3.chars().nth(i) {
                compare_result
            } else {
                iterator3.chars().nth(i)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        } else {
            let compare_result = if iterator1.chars().nth(i) > iterator2.chars().nth(i) {
                iterator1.chars().nth(i)
            } else {
                iterator2.chars().nth(i)
            };
            let result_char = if compare_result > iterator3.chars().nth(i) {
                compare_result
            } else {
                iterator3.chars().nth(i)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        }
        i += 1;
        position += 1
    }
    let result: String = array.iter().collect();
    result
}

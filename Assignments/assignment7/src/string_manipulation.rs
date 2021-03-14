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
    let mut itration: usize = 0;
    let mut array: Vec<char> = Vec::new();

    while itration < iterator2.len() {
        if position % 2 == 0 {
            let compare_result = if iterator1.chars().nth(itration) < iterator2.chars().nth(itration) {
                iterator1.chars().nth(itration)
            } else {
                iterator2.chars().nth(itration)
            };
            let result_char = if compare_result < iterator3.chars().nth(itration) {
                compare_result
            } else {
                iterator3.chars().nth(itration)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        } else {
            let compare_result = if iterator1.chars().nth(itration) > iterator2.chars().nth(itration) {
                iterator1.chars().nth(itration)
            } else {
                iterator2.chars().nth(itration)
            };
            let result_char = if compare_result > iterator3.chars().nth(itration) {
                compare_result
            } else {
                iterator3.chars().nth(itration)
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

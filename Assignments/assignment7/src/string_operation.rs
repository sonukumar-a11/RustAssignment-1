pub fn _desideratum_output(input_str1: &str, input_str2: &str, input_str3: &str) -> String {
    let mut count = 0;
    let mut i: usize = 0;
    let mut array: Vec<char> = Vec::new();
    while i < input_str2.len() {
        if count % 2 == 0 {
            let c = if input_str1.chars().nth(i) < input_str2.chars().nth(i) {
                input_str1.chars().nth(i)
            } else {
                input_str2.chars().nth(i)
            };
            let result_char = if c < input_str3.chars().nth(i) {
                c
            } else {
                input_str3.chars().nth(i)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        } else if count % 2 != 0 {
            // let resultant_char: &str = sorted_sending(input_str1[i],input_str2[i],input[i]);
            let c = if input_str1.chars().nth(i) < input_str2.chars().nth(i) {
                input_str2.chars().nth(i)
            } else {
                input_str1.chars().nth(i)
            };
            let result_char = if c < input_str3.chars().nth(i) {
                c
            } else {
                input_str3.chars().nth(i)
            };
            if let Some(_t) = result_char {
                array.push(result_char.unwrap());
            }
        }
        i += 1;
        count += 1;
    }
    let result: String = array.iter().collect();
    result
}

/// This function use to search pattern in given string
///
/// #Arguments
///
/// user_given_string - Given String to Apply to find pattern
/// search_pattern - checked with user_given_string
///
/// #Return
///
/// Returns the String to given value with index and match or not

pub fn _find_pattern(user_given_string: String, search_pattern: String) -> String {
    let search_store_iterable: Vec<char> = user_given_string.chars().collect();
    let store_pattern: Vec<char> = search_pattern.chars().collect();
    let mut iteration_count = 0;
    let mut pattern_check;
    let mut temp_index;
    let travel_iteration = search_store_iterable.len() - store_pattern.len() + 1;
    for index in 0..(travel_iteration) {
        temp_index = index;
        pattern_check = index;
        for index_match in &store_pattern {
            if index_match == &search_store_iterable[temp_index] {
                iteration_count += 1;
            }

            if iteration_count == search_pattern.len() {
                return pattern_check.to_string();
            }
            temp_index += 1;
        }
        iteration_count = 0;
    }
    "no match".to_string()
}

extern crate regex;

use std::collections::HashMap;

use regex::Regex;

/// conditional_sum which return the sum of ages for which the name contains the string
///
/// #Arguments
///
///key_value - A key_value is Hashmap object which contains the (string,integer)
///
///
/// #Return
///
/// Returns { Integer } having the sum of ages for which the name contains the string
pub fn conditional_sum(key_value: HashMap<&str, i32>, str: &str) -> i32 {
    let mut add_value = 0;
    let regex_val = Regex::new(&*(r"".to_string() + str)).unwrap();
    for item in key_value {
        if regex_val.is_match(item.0) {
            add_value += item.1
        }
    }
    log::info!("the sum of ages for which the name contains the string");
    add_value
}

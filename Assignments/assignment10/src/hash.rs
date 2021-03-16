extern crate regex;
use regex::Regex;
use std::collections::HashMap;
/// _sumcondtional return the sum of matching value in string
///
/// #Arguments
///
///key_value - Hashmap having key and value to manipulate the val;ue
///pattern_search = pattern search to consider the value present the value in hashmap
///
/// #Return
///
/// Returns i32 value to give the sum of value
pub fn _sumcondtional(key_value: HashMap<&str, i32>, str: &str) -> i32 {
    let mut add_value = 0;
    let regex_val = Regex::new(&*(r"".to_string() + str)).unwrap();
    for item in key_value {
        if regex_val.is_match(item.0) {
            add_value += item.1
        }
    }
    log::info!("Value : {}", add_value);
    add_value
}

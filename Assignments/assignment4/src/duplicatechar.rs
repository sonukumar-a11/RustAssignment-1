/// This function check the duplicate character in function.
///
/// #Arguments
///
/// iterable_data :- Take a Slice String as Required Argument and perform task to find duplicate in string
///
/// #Return
///
/// Return the string having duplicate value in present string.
pub fn find_duplicate(iterable_data: &str) -> String {
    let mut store_character_vector: Vec<char> = iterable_data.chars().collect();
    let mut record_index = 0;
    let mut stormers = String::new();
    while record_index < iterable_data.len() {
        let mut count_occupancy = 1;
        let mut next_index = record_index + 1;
        while next_index < iterable_data.len() {
            if store_character_vector[record_index] == store_character_vector[next_index]
                && store_character_vector[record_index] != ' '
            {
                count_occupancy += 1;
                store_character_vector[next_index] = '0';
            }
            next_index += 1;
        }
        if count_occupancy > 1 && store_character_vector[record_index] != '0' {
            stormers.push(store_character_vector[record_index]);
        }
        record_index += 1;
    }
    stormers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_duplicates_exist_0() {
        assert_eq!(find_duplicate("Sonu Kumar"), "u");
        assert_eq!(find_duplicate("Hello World"), "lo");
        assert_eq!(find_duplicate("Exist a Values"), "sa");
        assert_eq!(find_duplicate("Boat"), "");
    }
}

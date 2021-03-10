#[cfg(test)]
#[test]
fn check_string() {
    use crate::string_operation::_desideratum_output;
    let input1 = "jjdhid";
    let input2 = "ikjhjk";
    let input3 = "rtysgi";
    let output = _desideratum_output(input1, input2, input3);
    assert_eq!(output, "ikdhgi");

    assert_eq!(_desideratum_output("abcd", "efgh", "ijkl"), "afch");
}
#[test]
fn _generating_substring_here() {
    use crate::task::genreting_subset::_generating_substring;
    assert_eq!(
        _generating_substring("Sony".to_string()),
        ["S", "So", "Son", "Sony", "o", "on", "ony", "n", "ny", "y"]
    );
    assert_eq!(
        _generating_substring("One".to_string()),
        ["O", "On", "One", "n", "ne", "e"]
    );
}
#[test]
fn _find_pattern_here() {
    use crate::task::search_patter::_find_pattern;
    assert_eq!(
        _find_pattern("Maruti".to_string(), "ruti".to_string()),
        "2".to_string()
    );
    assert_eq!(
        _find_pattern("Knolway".to_string(), "Dell".to_string()),
        "no match".to_string()
    );
}

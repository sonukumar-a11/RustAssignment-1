#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::concept_with_question2::add_duplicate::duplicate_element;
    use crate::concept_with_question2::nth_value::Drop;
    use crate::concept_with_question2::palindrome_check::check_palindrome;
    use crate::concept_with_question2::remove_duplicate::compress;
    use crate::concept_with_question2::return_even::first_even;
    use crate::concept_with_question2::rev_iterable::reverse;
    use crate::hash_part::hash::conditional_sum;

    #[test]
    fn check_hash_sum() {
        use std::collections::HashMap;
        let mut ourmap = HashMap::new();
        ourmap.insert("sonu", 15);
        ourmap.insert("aadarsh", 16);
        ourmap.insert("monu", 94);
        ourmap.insert("shubranu", 4);
        ourmap.insert("satyendra", 21);
        assert_eq!(conditional_sum(ourmap, "nu"), 113);
    }

    #[test]
    fn check_hash_sum_next() {
        let mut usermap = HashMap::new();
        usermap.insert("Rekha", 65);
        usermap.insert("Reksh", 36);
        usermap.insert("djekfth", 14);
        usermap.insert("shubranu", 4);
        usermap.insert("satyendra", 21);
        assert_eq!(conditional_sum(usermap, "ek"), 115);
    }

    #[test]
    fn palindrome_check() {
        let output = vec![1, 1];
        assert_eq!(check_palindrome(output), true);
    }

    #[test]
    fn palindrome_check_next() {
        assert_eq!(check_palindrome(vec![1, 2]), false);
    }

    #[test]
    fn check_reverse() {
        let mut output = vec![1, 1];
        assert_eq!(reverse(&mut output), [1, 1]);
    }

    #[test]
    fn check_reverse_next() {
        let mut output = vec![1, 3, 4];
        assert_eq!(reverse(&mut output), [4, 3, 1]);
    }

    #[test]
    fn check_drop() {
        let mut drop_iterable: Drop = Drop {
            nth_value: 3,
            iterable: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(drop_iterable.drop_element(), [1, 2, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn check_drop_next() {
        let mut drop_iterable: Drop = Drop {
            nth_value: 6,
            iterable: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        assert_eq!(drop_iterable.drop_element(), [1, 2, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn summation_duplicate_check() {
        let mut output = vec![1, 2];
        assert_eq!(duplicate_element(&mut output), [1, 1, 2, 2]);
    }

    #[test]
    fn summation_duplicate_check_next() {
        let mut output = vec![1, 2, 3, 4];
        assert_eq!(duplicate_element(&mut output), [1, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn check_even() {
        let output = vec![1, 2];
        assert_eq!(first_even(output), 2);
    }

    #[test]
    fn check_even_next() {
        let output = vec![1, 0];
        assert_eq!(first_even(output), 0);
    }

    #[test]
    fn check_duplicate_exist() {
        let output = vec![0, 0, 1, 1];
        assert_eq!(compress(output), [0, 1]);
    }

    #[test]
    fn check_duplicate_exist_next() {
        let output = vec![0, 0, 1, 1, 2, 2, 3, 4, 4, 5];
        assert_eq!(compress(output), [0, 1, 2, 3, 4, 5]);
    }
}

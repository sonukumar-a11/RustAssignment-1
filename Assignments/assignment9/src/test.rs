#[cfg(test)]
mod tests {
    use crate::assosiate_function::minimum_value::min_value;
    use crate::assosiate_function::sort_something::sort_elements;
    use crate::custom_iterable::{GeometricSeries, Iterator};

    #[test]
    fn check_custom_iterator() {
        let mut gp = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            gp.take(11),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn check_custom_iterator_next() {
        let mut gp = GeometricSeries {
            first_number: 0,
            current_number: 0,
            ratio: 0,
        };
        assert_eq!(gp.take(1), [0]);
    }

    #[test]
    fn check_sort() {
        let mut array: [i32; 4] = [2, 3, 1, 4];
        assert_eq!(sort_elements(&mut array), [1, 2, 3, 4]);
    }

    #[test]
    fn check_sort_next() {
        let mut array: [i32; 3] = [9, 8, 5];
        assert_eq!(sort_elements(&mut array), [5, 8, 9])
    }

    #[test]
    fn find_min_value_for_int() {
        assert_eq!(min_value(1, 2), 1);
    }

    #[test]
    fn find_min_value_for_char() {
        assert_eq!(min_value('a', 'f'), 'a');
    }

    #[test]
    fn find_min_value_for_float() {
        assert_eq!(min_value(0.0, 1.1), 0.0);
    }
}

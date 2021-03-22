#[cfg(test)]
mod tests {
    use crate::associate_function::minimum_value::min_value;
    use crate::associate_function::sort_something::sort_elements;
    use crate::custom_iterable::{GeometricSeries, Iterator};

    #[test]
    fn check_custom_iterator() {
        let mut gp = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(
            gp.take(11).unwrap(),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn check_custom_iterator_invalid() {
        let mut gp = GeometricSeries {
            first_number: 1,
            current_number: 1,
            ratio: 2,
        };
        assert_eq!(gp.take(0), Err("Please provide valid input".to_owned()));
    }

    #[test]
    fn check_custom_iterator_next() {
        let mut gp = GeometricSeries {
            first_number: 0,
            current_number: 0,
            ratio: 0,
        };
        assert_eq!(gp.take(1).unwrap(), [0]);
    }

    #[test]
    fn check_sort_int() {
        let mut arr: [i32; 3] = [7, 9, 0];
        assert_eq!(sort_elements(&mut arr).unwrap(), [0, 7, 9]);
    }

    #[test]
    fn check_sort_float() {
        let mut arr: [f32; 5] = [3.2, 2.2, 5.2, 1.2, 4.2];
        assert_eq!(sort_elements(&mut arr).unwrap(), [1.2, 2.2, 3.2, 4.2, 5.2]);
    }

    #[test]
    fn check_sort_char() {
        let mut arr: [char; 5] = ['k', 'h', 'j', 'i', 'm'];
        assert_eq!(sort_elements(&mut arr).unwrap(), ['h', 'i', 'j', 'k', 'm']);
    }

    #[test]
    fn find_min_value_for_int() {
        assert_eq!(min_value(1, 2), Ok(1));
    }

    #[test]
    fn find_min_value_for_equal() {
        assert_eq!(
            min_value(1, 1),
            Err("Given piece are input / invalid input".to_owned())
        );
    }

    #[test]
    fn find_min_value_for_char() {
        assert_eq!(min_value('a', 'f'), Ok('a'));
    }

    #[test]
    fn find_min_value_for_float() {
        assert_eq!(min_value(0.0, 1.1), Ok(0.0));
    }
}

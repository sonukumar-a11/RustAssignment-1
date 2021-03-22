#[cfg(test)]
mod tests {
    use crate::apply_feature::index_element::nth;
    use crate::apply_feature::repeat_finder::first_repeated;
    use crate::apply_feature::repeated_number::second_repeated;
    use crate::apply_feature::third_odd_number::third_odd;
    use crate::datastore::Store::{Nil, Value};

    #[test]
    fn first_repeated_check() {
        env_logger::init();
        let list_val = Value(
            1,
            Box::new(Value(
                21,
                Box::new(Value(
                    21,
                    Box::new(Value(
                        4,
                        Box::new(Value(5, Box::new(Value(5, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeated(list_val), Ok(21));
    }
    #[test]
    fn first_repeated_check_next_times() {
        let list_val = Value(
            1,
            Box::new(Value(
                21,
                Box::new(Value(
                    1,
                    Box::new(Value(
                        4,
                        Box::new(Value(5, Box::new(Value(7, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeated(list_val), Ok(-1));
    }
    #[test]
    fn second_repeat_check() {
        let box_array = Value(
            1,
            Box::new(Value(
                21,
                Box::new(Value(
                    21,
                    Box::new(Value(
                        4,
                        Box::new(Value(7, Box::new(Value(7, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeated(box_array), Ok(7));
    }

    #[test]
    fn second_repeat_check_next_time() {
        let box_array = Value(
            1,
            Box::new(Value(
                21,
                Box::new(Value(
                    21,
                    Box::new(Value(
                        4,
                        Box::new(Value(7, Box::new(Value(9, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeated(box_array), Ok(-1));
    }

    #[test]
    fn element_search_check() {
        let test_list = Value(
            1,
            Box::new(Value(
                2,
                Box::new(Value(
                    3,
                    Box::new(Value(
                        4,
                        Box::new(Value(5, Box::new(Value(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth(4, test_list), Ok(4));
    }

    #[test]
    fn element_search_check_next_time() {
        let test_list = Value(
            1,
            Box::new(Value(
                2,
                Box::new(Value(
                    3,
                    Box::new(Value(
                        4,
                        Box::new(Value(5, Box::new(Value(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(nth(7, test_list), Ok(-1));
    }

    #[test]
    fn third_odd_check() {
        let test_data = Value(
            1,
            Box::new(Value(
                21,
                Box::new(Value(
                    3,
                    Box::new(Value(
                        4,
                        Box::new(Value(5, Box::new(Value(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(third_odd(test_data), Ok(3));
    }

    #[test]
    fn third_odd_check_next_time() {
        let test_data = Value(
            1,
            Box::new(Value(
                2,
                Box::new(Value(
                    3,
                    Box::new(Value(
                        4,
                        Box::new(Value(8, Box::new(Value(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(third_odd(test_data), Ok(-1));
    }
}

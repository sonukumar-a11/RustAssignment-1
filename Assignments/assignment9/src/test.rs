#[cfg(test)]
#[test]
fn check_question() {
    use crate::custom_iterable::{GeometricSeries, Iterator};
    let mut gp = GeometricSeries {
        first_number: 1,
        current_number: 1,
        ratio: 2,
    };
    assert_eq!(
        gp.take(11),
        vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );

    let mut gp = GeometricSeries {
        first_number: 0,
        current_number: 0,
        ratio: 0,
    };
    assert_eq!(gp.take(1), [0]);
}

#[test]
fn _check_sort() {
    use crate::sort_something::_sort_anything;
    let mut array: [i32; 4] = [2, 3, 1, 4];
    assert_eq!(_sort_anything(&mut array), [1, 2, 3, 4]);
    let mut array: [i32; 3] = [9, 8, 5];
    assert_eq!(_sort_anything(&mut array), [5, 8, 9])
}

#[test]
fn _find_min_value() {
    use crate::minimum_value::_min_value;
    assert_eq!(_min_value(1, 2), 1);
    assert_eq!(_min_value('a', 'f'), 'a');
    assert_eq!(_min_value(0.0, 1.1), 0.0);
}

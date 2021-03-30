#[cfg(test)]
mod tests {
    use crate::coordinate_check::check_coordinates::{check_coordinate, Coordinate, Position};
    use crate::ipcheck::check_ipaddress::{check_ip_address, IpAddress};

    #[test]
    fn check_coordinate_for_first() {
        let point_check = (2, 3);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::First(
                Coordinate::Abscissa(2),
                Coordinate::Ordinate(3),
            ))
        )
    }

    #[test]
    fn check_coordinate_for_second() {
        let point_check = (-2, 3);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::Second(
                Coordinate::Abscissa(-2),
                Coordinate::Ordinate(3),
            ))
        )
    }

    #[test]
    fn check_coordinate_for_third() {
        let point_check = (-2, -3);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::Third(
                Coordinate::Abscissa(-2),
                Coordinate::Ordinate(-3),
            ))
        )
    }

    #[test]
    fn check_coordinate_for_fourth() {
        let point_check = (2, -3);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::Fourth(
                Coordinate::Abscissa(2),
                Coordinate::Ordinate(-3),
            ))
        )
    }

    #[test]
    fn check_coordinate_for_x_axis() {
        let point_check = (-2, 0);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::XAxis(
                Coordinate::Abscissa(-2),
                Coordinate::Ordinate(0),
            ))
        )
    }

    #[test]
    fn check_coordinate_for_y_axis() {
        let point_check = (0, 3);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::YAxis(
                Coordinate::Abscissa(0),
                Coordinate::Ordinate(3),
            ))
        )
    }

    #[test]
    fn check_coordinate_next() {
        let point_check = (0, 0);
        assert_eq!(
            check_coordinate(point_check),
            Ok(Position::Origin(
                Coordinate::Abscissa(0),
                Coordinate::Ordinate(0),
            ))
        );
    }

    #[test]
    fn check_ip_class_for_a() {
        assert_eq!(
            check_ip_address((102, 1, 3, 4)),
            Ok(IpAddress::ClassA("102.1.3.4".to_string()))
        );
    }

    #[test]
    fn check_ip_class_for_b() {
        assert_eq!(
            check_ip_address((143, 143, 10, 10)),
            Ok(IpAddress::ClassB("143.143.10.10".to_string()))
        );
    }

    #[test]
    fn check_ip_class_for_c() {
        assert_eq!(
            check_ip_address((220, 143, 3, 4)),
            Ok(IpAddress::ClassC("220.143.3.4".to_string()))
        );
    }

    #[test]
    fn check_ip_class_for_d() {
        assert_eq!(
            check_ip_address((231, 1, 3, 4)),
            Ok(IpAddress::ClassD("231.1.3.4".to_string()))
        );
    }

    #[test]
    fn check_ip_class_for_e() {
        assert_eq!(
            check_ip_address((246, 1, 3, 4)),
            Ok(IpAddress::ClassE("246.1.3.4".to_string()))
        );
    }

    #[test]
    fn check_ip_class_for_invalid() {
        assert_eq!(
            check_ip_address((102, 26666, 2589, 6589)),
            Err("Unwanted Input".to_owned())
        );
    }
}

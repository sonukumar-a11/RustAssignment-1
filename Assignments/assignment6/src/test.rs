#[cfg(test)]
use crate::check_coordinates::_Point;

#[test]
fn check_coordinate_axis0() {
    let point: Vec<_Point> = vec![_Point(3, 4), _Point(-4, 3), _Point(-3, -4), _Point(3, -4)];
    let output1 = point[0]._check_coordinate();
    assert_eq!(output1, "Abscissa 3 , Ordinate 4 ,First_Quadrant");

    let output2 = point[1]._check_coordinate();
    assert_eq!(output2, "Abscissa -4 , Ordinate 3 ,Second_Quadrant");

    let output3 = point[2]._check_coordinate();
    assert_eq!(output3, "Abscissa -3 , Ordinate -4 ,Third_Quadrant");

    let output4 = point[3]._check_coordinate();
    assert_eq!(output4, "Abscissa 3 , Ordinate -4 ,Fourth_Quadrant");
}

#[test]
fn check_ip() {
    use crate::check_ipaddress;
    use crate::check_ipaddress::IpAddress;
    let ip_address_input = vec![
        (121, 0, 1, 0),
        (132, 0, 1, 1),
        (198, 0, 1, 1),
        (245, 0, 1, 1),
        (266, 0, 1, 1),
    ];
    let output_ip_address = check_ipaddress::_check_ip(ip_address_input[0]);
    assert_eq!(output_ip_address, IpAddress::_ClassA("Class A".to_string()));

    let output_ip_address1 = check_ipaddress::_check_ip(ip_address_input[1]);
    assert_eq!(
        output_ip_address1,
        IpAddress::_ClassB("Class B".to_string())
    );

    let output_ip_address2 = check_ipaddress::_check_ip(ip_address_input[2]);
    assert_eq!(
        output_ip_address2,
        IpAddress::_ClassC("Class C".to_string())
    );

    let output_ip_address3 = check_ipaddress::_check_ip(ip_address_input[3]);
    assert_eq!(
        output_ip_address3,
        IpAddress::_ClassD("Class D".to_string())
    );
}

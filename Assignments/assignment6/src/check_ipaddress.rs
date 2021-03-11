#[derive(PartialEq, Eq, Debug)]
/// This  Tuple Structs without Named Fields to Create Point
///
/// #Arguments
///
/// Self :- Take a Point of coordinate  (x,y)
///
///  #Return
/// Enum
pub enum IpAddress {
    _ClassA(String),
    _ClassB(String),
    _ClassC(String),
    _ClassD(String),
    _NotValid(String),
}
/// This method Used to find the Class of IPV4 version IP Address
///
/// #Arguments
///
/// Self :- Take a ipconfig using tuple Structure
///
///  #Return
/// this method return class of ipAddress.
pub fn _check_ip(ipconfig: (i32, i32, i32, i32)) -> IpAddress {
    let class1: &str = "Class A";
    let class2: &str = "Class B";
    let class3: &str = "Class C";
    let class4: &str = "Class D";
    let class5: &str = "Not Valid";

    match ipconfig {
        (a, _, _, _) if (1..=127).contains(&a) => IpAddress::_ClassA(class1.to_string()),
        (a, _, _, _) if (128..=191).contains(&a) => IpAddress::_ClassB(class2.to_string()),
        (a, _, _, _) if (192..=223).contains(&a) => IpAddress::_ClassC(class3.to_string()),
        (a, _, _, _) if (224..=255).contains(&a) => IpAddress::_ClassD(class4.to_string()),
        _ => IpAddress::_NotValid(class5.to_string()),
    }
}

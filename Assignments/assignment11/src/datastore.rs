#[derive(PartialEq, Eq, Debug)]
///IpAddress enum which used to encapsulate the class of ipAddress
///
/// #field
///
///Value(i32, Box<Store>) : - A value is tuple object having {integer} and instance somewhere on the heap, managed and owned by the Box object.
///
///  Nil:- Signal the ends of the list
pub enum Store {
    Value(i32, Box<Store>),
    Nil,
}

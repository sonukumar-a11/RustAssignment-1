pub enum Store {
    Value(i32, Box<Store>),
    Nil,
}

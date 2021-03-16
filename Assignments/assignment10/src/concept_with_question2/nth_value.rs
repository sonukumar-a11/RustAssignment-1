/// _Drop Structure used to store the field and bind to use as custom dataType
///
/// #Arguments
///
///nth_value - Take a index as in usize
///iterable - take a iterable to manipulation on iterable
///
/// #Return
///
/// Returns the bind into single unit
pub struct _Drop {
    pub nth_value: usize,
    pub iterable: Vec<i32>,
}
impl _Drop {
    /// _drop_element function remove the Drop.index value from given iterable
    ///
    /// #Arguments
    ///
    ///self:- take a structure data
    ///
    /// #Return
    ///
    /// Returns Vector to remove the Drop.index value from given iterable
    pub fn _drop_element(&self) -> Vec<i32> {
        let mut index: usize = 0;
        let mut answer: Vec<i32> = Vec::new();
        loop {
            if index == self.nth_value - 1 {
                answer.extend_from_slice(&self.iterable[..index]);
                break;
            }
            index += 1
        }
        answer.extend_from_slice(&self.iterable[index + 1..]);
        answer
    }
}

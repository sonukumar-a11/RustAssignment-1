use std::convert::TryInto;

///Drop structure which used to encapsulate the data
///
/// #field
///
/// nth_value:-a nth_value is usize object consider as remove value
///
/// iterable:-A iterable is Vector object which contains the list of numbers
pub struct Drop {
    pub nth_value: usize,
    pub iterable: Vec<i32>,
}

impl Drop {
    /// duplicate_element which drop the nth element
    ///
    /// #Arguments
    ///
    ///iterable - A iterable is Vector object which contains the list of numbers
    ///
    /// #Return
    ///
    /// Returns Result<T,String> T consider as a give a valid result and handle the error as well........
    pub fn drop_element(&mut self) -> Result<Vec<i32>, String> {
        if self.iterable.is_empty() {
            return Err("Iterable is not valid".to_string());
        }
        let mut iteration: usize = 0;
        while iteration < self.iterable.len() {
            if self.nth_value == self.iterable[iteration].try_into().unwrap() {
                self.iterable.remove(iteration);
            }

            iteration += 1;
        }

        let array;
        array = self.iterable.clone();
        Ok(array)
    }
}

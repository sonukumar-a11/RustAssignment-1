/// GeoProgression Structure
///
/// #Arguments
///
///first_number: take input from user
///current_number
/// #Return
///
/// Return  the sorted way....
pub trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn take(&mut self, size: i32) -> Vec<i32>;
}

pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeometricSeries {
    fn next(&mut self) -> Option<i32> {
        let out = self.current_number;
        self.current_number *= self.ratio;
        Some(out)
    }
    /// take function generates an iterator for n elements in GP.
    ///
    /// #Arguments
    ///
    /// count -required number you want to generate
    ///
    /// #Return
    ///
    /// Returns the Vector containing count times in vector..
    fn take(&mut self, count: i32) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for _index in 0..count {
            out.push(self.next().unwrap());
        }
        out
    }
}

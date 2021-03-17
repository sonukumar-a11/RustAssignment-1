pub trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn take(&mut self, size: i32) -> Vec<i32>;
}
///GeometricSeries structure which used to encapsulate the data
///
/// #field
///
/// first_number:-a first_number is integer object consider as first term in Series
///current_number :- a current_number is integer object consider as current_term by which you can find preceding term  in Series
/// ratio:A ration is integer value consider as common ration

pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeometricSeries {
    /// next which which is used to solve the preceding term
    ///
    /// #Arguments
    ///
    ///self - A self means called method through structure as parameter
    ///
    /// #Return
    ///
    /// Returns Option type enum  and containing preceding term
    fn next(&mut self) -> Option<i32> {
        let out = self.current_number;
        self.current_number *= self.ratio;
        Some(out)
    }
    /// take which is used automatic solve through count term
    ///
    /// #Arguments
    ///
    /// count -A count is {integer} object used to solve next t next preceding term
    ///
    /// #Return
    ///
    /// Returns Vector collect the next count as preceding term
    fn take(&mut self, count: i32) -> Vec<i32> {
        let mut out: Vec<i32> = Vec::new();
        for _index in 0..count {
            out.push(self.next().unwrap());
        }
        out
    }
}

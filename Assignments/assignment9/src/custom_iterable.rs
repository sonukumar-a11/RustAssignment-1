use ::log::*;

pub trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn take(&mut self, size: i32) -> Result<Vec<i32>, String>;
}

///GeometricSeries structure which used to encapsulate the data
///
/// #field
///
/// first_number:-a first_number is integer object consider as first term in Series
///
///current_number :- a current_number is integer object consider as current_term by which you can find next preceding term  in Series
///
/// ratio:A ration is integer value consider as common ration
///
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
    /// Returns Option<T> type enum  and containing preceding term and handle error as well
    fn next(&mut self) -> Option<i32> {
        let result: Option<i32> = Some(self.current_number);
        self.current_number *= self.ratio;
        match result {
            Some(result) => Option::from(result),
            None => panic!("Due to putting wrong current number"),
        }
    }
    /// take which is used automatic solve through count term
    ///
    /// #Arguments
    ///
    /// count -A count is {integer} object used to solve next t next preceding term
    ///
    /// #Return
    ///
    /// Returns Result<T,E> which contains as Store the next count as preceding term and handle Error as Well...
    fn take(&mut self, count: i32) -> Result<Vec<i32>, String> {
        if count == 0 {
            error!("Did not understand");
            return Err("Please provide valid input".to_string());
        }

        let mut series_value: Vec<i32> = Vec::new();
        for _index in 0..count {
            series_value.push(self.next().unwrap());
        }
        info!("Good choice: {:?}", series_value);
        Ok(series_value)
    }
}

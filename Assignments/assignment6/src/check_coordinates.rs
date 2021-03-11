/// This  Tuple Structs without Named Fields to Create Point
///
/// #Arguments
///
/// Self :- Take a Point of coordinate  (x,y)
///
///  #Return
/// tuple Structure

pub struct _Point(pub i32, pub i32);

impl _Point {
    /// This method Used to find the Quadrant in which Particular Point lies
    ///
    /// #Arguments
    ///
    /// Self :- Take a Point of coordinate  using tuple Structure
    ///
    ///  #Return
    /// this method return Quadrant of Point in
    pub fn _check_coordinate(&self) -> String {
        let x = self.0;
        let y = self.1;
        let first_quadrants: &str = "First_Quadrant";
        let second_quadrant: &str = "Second_Quadrant";
        let third_quadrant: &str = "Third_Quadrant";
        let fourth_quadrant: &str = "Fourth_Quadrant";
        match (x, y) {
            (x, y) if x > 0 && y > 0 => {
                format!("Abscissa {} , Ordinate {} ,{}", x, y, first_quadrants)
            }
            (x, y) if x < 0 && y > 0 => {
                format!("Abscissa {} , Ordinate {} ,{}", x, y, second_quadrant)
            }
            (x, y) if x < 0 && y < 0 => {
                format!("Abscissa {} , Ordinate {} ,{}", x, y, third_quadrant)
            }
            _ => {
                format!("Abscissa {} , Ordinate {} ,{}", x, y, fourth_quadrant)
            }
        }
    }
}

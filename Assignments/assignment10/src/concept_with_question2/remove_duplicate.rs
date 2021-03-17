/// return_unique which Remove continuously occurring duplicate
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns Vector which maintain the Continuously unique element  
pub fn compress(iterable: Vec<i32>) -> Vec<i32> {
    let mut unique: Vec<i32> = Vec::new();
    for item in iterable {
        let top = unique.last();
        match top {
            Some(value) => {
                if item != *value {
                    unique.push(item);
                }
            }
            None => unique.push(item),
        }
    }
    log::info!("Remove continuously occurring duplicate");
    unique
}

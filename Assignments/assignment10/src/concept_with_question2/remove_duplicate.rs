/// return_unique which Remove continuously occurring duplicate
///
/// #Arguments
///
///iterable - A iterable is Vector object which contains the list of numbers
///
/// #Return
///
/// Returns Result<Vec<T>,String> which maintain the Continuously unique element and Handle error as well
pub fn compress(iterable: Vec<i32>) -> Result<Vec<i32>, String> {
    if iterable.is_empty() {
        return Err("Iterable is not valid".to_string());
    }
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
    Ok(unique)
}

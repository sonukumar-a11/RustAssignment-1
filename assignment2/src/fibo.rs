use  text_io::read;

pub fn fibonacci()
{
    let f:i32 = read!();
    // let c= f;
    // let mut arr:[i32;c];
    let mut first = 0;
    let mut  next = 1;
    let mut calculate:i32 ;
    
    let mut index = 0;

    while index < f{
        calculate = first+next;
        first = next;
        next = calculate;
        println!("{}",first);
        index+=1;
    }
}
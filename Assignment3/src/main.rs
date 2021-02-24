use text_io::read;

pub mod linear_bineary;

pub mod merge_sort;
mod chack_leap;

fn main() {
    println!("This is done by Sonu");

    //linearBInarySearch
    //take the size of element

    //for linear search

    // let f = read!();
    // let mut size: i32 = read!();
    // let k = externalheap::HeapArray::new(f);
    println!("enter decision for apply which you want \
    1:- Binary Search \
    2:- LinearSearch \
    3:- mergeSort \
    4:- Leap");
    let decide: i32 = read!();

    let arr1: [i32; 5] = [10, 20, 30, 40, 50];
    let arr2: [i32; 5] = [10, 30, 60, 80, 50];
    let arr3 = [10, 30, 60, 80, 50];
    //for search
    if decide == 1 || decide == 2
    {
        println!("Search for decision 1 and 2");
        let x: i32 = read!();
        if decide == 1
        {
            println!("please Give Sort Array for binary Search");

            linear_bineary::bineary_search(&arr1, x, 0, 5);
        }
        //for linear scearch
        else if decide == 2
        {
            linear_bineary::lsearch(&arr2, x, 0);
        }
    }
    else {
        if decide == 3 {
           chack_leap::count_leap();
       }
        else {
            merge_sort::mergesortmain(&arr3);
        }
    }
}
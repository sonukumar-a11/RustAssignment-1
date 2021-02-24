// use super::externalheap;
//why this decleration beacuse the sae externlhap structure which is prsent on other file
pub fn lsearch(arr:&[i32], x: i32, index: i32)
{
    if arr.len() == index as usize
    {
        println!("Number is not present");
        return;
    }
    if arr[index as usize] == x 
    {   
        println!("{}" , index);
        return ;
    }
    lsearch(arr, x, index+1);
}


pub fn bineary_search(arr: &[i32], x: i32, si: i32, ei: i32)
{
    if si > ei{
        return;
    }

    //for find element
    let mid = si+ei>>1;

    if arr[mid as usize] == x 
    {
        println!("present at index: {}",mid);
        return ;
    }
    else if arr[mid as usize] < x{
        bineary_search(arr, x, mid+1, ei);

    }
    else
    {
        bineary_search(arr, x, si, mid-1);
    }
}
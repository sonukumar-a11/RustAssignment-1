mod duplicatechar;
mod is_plaindrome;
mod rotate_string;

fn main() {
    println!(
        "Strings : {}",
        rotate_string::check_rotation("abcd", "dcba")
    );
    let variable: &str = "abba";
    println!("String is : {}", is_plaindrome::check_palindrome(variable));
    println!(
        "Duplicate Characters : {}",
        duplicatechar::find_duplicate("Hello World")
    );
}

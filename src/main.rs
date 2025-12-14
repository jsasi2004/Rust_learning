use std::mem;
fn main() {
    let _string = "hello world"; // unused so _ at the beginning
    let _my_num: i32 = 5;          // integer
    let _my_double: f64 = 5.99;    // float
    let _my_letter: char = 'D';    // character
    let _my_bool: bool = true;     // boolean
    let _my_text: &str = "Hello";  // string
    let my_num_64: i64 = 8;
    println!("{}",my_num_64);
    println!("{}",_my_bool);
    println!("{}",mem::size_of_val(&_my_double));
    println!("{}",mem::size_of::<i32>());
}

/*
This is a shorter way to read the string given by the userand perform the desired action.
This method uses an external package called text_io which has a property called "read", this will help us to shorten out code.

For users using multiple inputs on the terminal and are also looking for a shorter way to read lines the text_io pakage is recommended
*/

use text_io::read;

fn main() {
    
    // Float input
    println!("Enter a number (f64): ");
    let float_num: f64 = read!();
  
    // Integer input
    println!("Enter an integer (i32): ");
    let int_num: i32 = read!();
    
    // String input
    println!("Enter a string: ");
    let str_input: String = read!();

    println!("You entered a number: {}", float_num);
    println!("You entered an integer: {}", int_num);
    println!("You entered a string: {}", str_input);
}

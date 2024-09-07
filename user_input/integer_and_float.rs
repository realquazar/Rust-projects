/*
In Rust the input system is provided by the std::io package.
All inputs read are stored in the String datatype by default.
For integers or doubles we have to parse them and store it in a seperate variable to access it similar to C++/java.
*/

use std::io;

fn main() {
    
    // Create a new empty string for the input
    println!("Enter an integer: ");    
    let mut input = String::new();
    
    // Read the entire line and store it in a mutable reference to input
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    // Parsing the string to return the integer value of the string
    let int_num: i32 = input.trim().parse()
        .expect("Invalid integer");

    // We can now convert the string to a float/double (decimal)
    let decimal_num: f64 = int_num as f64;

    println!("The integer representation is: {}", int_num);
    println!("The decimal representation is: {}", decimal_num);
}

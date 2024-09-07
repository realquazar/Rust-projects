// Here we accept a string and display it using the io package incuded in rusts standard (std) library

use std::io;

fn main() {
    println!("Enter a string: ");

    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input)
        .expect("Failed to read line");

    println!("You entered: {}", str_input.trim());
}

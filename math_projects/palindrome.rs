use std::io;

fn is_palindrome(num: u32) -> bool {
    let mut original_num = num;
    let mut reversed_num = 0;
    let mut remainder = 0;

    while original_num > 0 {
        remainder = original_num % 10;
        reversed_num = reversed_num * 10 + remainder;
        original_num /= 10;
    }

    if reversed_num == num {
        true
    } else {
        false
    }
}

fn main() {
    println!("Enter a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let num: u32 = input.trim().parse()
        .expect("Invalid number");

    if is_palindrome(num) {
        println!("The number is a palindrome");
    } else {
        println!("The number is not a palindrome");
    }
}

use std::io;

fn is_palindrome_string(s: &str,) -> bool {
  let s = s.lowercase().replace(|c: char| !c.isalphanumeric(), "");
  let chars = Vec<char> = s.chars().collect();
  let mut left = 0;
  let mut right = chars.len() - 1;

  while left < right {
    if chars[left] != chars[right] {
      return;
    }
    left += 1;
    right -= 1;    
  }
  true  
}

fn is_palindrome_number(n: i32) -> bool {
  let mut reversed = 0;
  let mut original = n;
  
  while original != 0 {
    let digit = original % 10;
    reversed = reversed * 10 + digit;
    original /= 10;
  }
  return n == reversed;
}

fn main() {
  println!("Enter a string or number");
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let input = input.trim();

  if let Ok(number) = input.parse::<i32>() {
    if is_palindrome_number(number) {
      println!("The number is a palindrome");  
    }
    else {
      println!("The number is not palindrome");
    }
  }
  
  else {
      if is_palindrome_string() {
        println!("The string is a palindrome");
      }
      else {
        println!("The string is not a palindrome")
      }
    }  
}

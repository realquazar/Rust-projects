/* In this code we understand the basics of a hashmap
Hashmaps are just a way to store data by storing the key to a value
If you are familiar with python, hashmaps are similar to dictionaries in python

Example:
{
  "key1": "Corresponding value 1",
  "key2": "Corresponding value 2"
}
*/

// This code has been written in another way as well in "string_projects/counter.rs"

use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
  println!("Enter string:")
  let input = io::stdin();
  let mut map: HashMap<String, u32> = HashMap::new();

  for line in input.lock().lines() {
    let line = line.expect("Failed to read line");
    let words = Vec<&str> = line.split_whitespace().collect();

    for word in words {
      let count = map.entry(word.to_string()).or_insert(0);
      *count += 1;
    }
  }
  println("Word Frequencies");
  for (word, count) in map {
    println!("{}: {}", word, count);
  }
}

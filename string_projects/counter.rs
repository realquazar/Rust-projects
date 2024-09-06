use std::collections::HashMap;

fn main {
  let word = "Hello!";
  let mut count_map = Hashmap::new();

  for c in word.to_lowercase().chars() {
    let count = count_map.entry(c).or_insert(0);
    *count += 1;
  }

  for (c, count) in count_map {
    if c.is_alphabetic() {
      println!("Letter {}: {}", c, count);
    }
    else {
      println!("Symbol {}: {}", c, count);
    }
  }
}

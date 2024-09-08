/* 
This program returns the content of each line stored in content.txt using Rusts file system package included in its standard library
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("content.txt").expect("Unable to read file");
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        println!("Line {}: {}", index + 1, line);
    }
}

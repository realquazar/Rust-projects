// In this code we accept 3 angles from the user and check if it forms a triangle

use std::io;

fn main() {
    let mut angles: Vec<u32> = Vec::new();

    for i in 1..4 {
        println!("Enter angle {} (in degrees): ", i);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let angle: u32 = input.trim().parse()
            .expect("Invalid angle");
        angles.push(angle);
    }

    if angles[0] + angles[1] + angles[2] == 180 {
        println!("The angles form a triangle");
    } else {
        println!("The angles do not form a triangle");
    }
}

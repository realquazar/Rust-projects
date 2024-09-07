use std::io;

fn main() {
    println!("Enter a number:")
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };
    
    if num <= 1 {
        println("{} is not a prime number", num);
    }
    else if num == 2 {
        println("{} is a prime number", num);        
    }
    else {
        let mut is_prime = true;

        for i in 2..((num as f64).sqrt() as u32 + 1) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            println!("{} is a prime number", num);
        }
        else {
            println!("{} is not a prime number", num);
        }
    }
}

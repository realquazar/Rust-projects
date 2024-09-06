use std::io;

fn main() {
  const NUM: i32 = 5;

  loop {
    println!("Guess a number from 1-10");

    let mut guess = String::new();

    io::stdin()
      .readline(&mut guess)
      .expect("Failed to read line");

    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input, please enter a number")
        continue;
      }      
    };
    
    if guess < 1 || guess > 10 {
      println!("Number out of range");
    }
    else if guess == NUM {
      println!("Congratulations! You guessed the number!");
      break;
    }
    else {
      println!("That wasn't the number, try again!")
    }
  }
}

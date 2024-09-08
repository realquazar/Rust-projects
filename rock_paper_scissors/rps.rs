use text_io::read;

fn main() {
    println!("Welcome to Rock Paper Scissors!");

    loop {
        println!("Please enter your choice (rock, paper, or scissors): ");
        let user_choice = read!("{}\n");

        let user_choice = user_choice.trim().to_lowercase();

        if user_choice != "rock" && user_choice != "paper" && user_choice != "scissors" {
            println!("Invalid choice. Please enter rock, paper, or scissors.");
            continue;
        }

        let computer_choice = get_computer_choice();

        println!("You chose: {}", user_choice);
        println!("Computer chose: {}", computer_choice);

        let result = get_result(user_choice, computer_choice);
        println!("{}", result);

        println!("Do you want to play again? (yes/no)");
        let play_again = read!("{}\n");

        let play_again = play_again.trim().to_lowercase();

        if play_again != "yes" {
            break;
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let choice = choices[rand::thread_rng().gen_range(0..3)];
    choice.to_string()
}

fn get_result(user_choice: String, computer_choice: String) -> String {
    if user_choice == computer_choice {
        return "It's a tie!".to_string();
    }

    match user_choice.as_str() {
        "rock" => {
            if computer_choice == "scissors" {
                return "Rock smashes scissors! You win!".to_string();
            } else {
                return "Paper covers rock! You lose!".to_string();
            }
        }
        "paper" => {
            if computer_choice == "rock" {
                return "Paper covers rock! You win!".to_string();
            } else {
                return "Scissors cuts paper! You lose!".to_string();
            }
        }
        "scissors" => {
            if computer_choice == "paper" {
                return "Scissors cuts paper! You win!".to_string();
            } else {
                return "Rock smashes scissors! You lose!".to_string();
            }
        }
        _ => unreachable!(),
    }
}

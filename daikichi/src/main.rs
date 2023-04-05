use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    let daikichis = vec![
        "You will find love today!",
        "You will find great fortune in the near future.",
        "You will find relief from the everyday in an extravagent way!",
        "Today will be like all others, long and boring",
        "Good luck....",
    ];

    println!("Welcome to the Daikichi Creator!");
    println!("Press enter to generate your fortune...");

    loop {
        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim();

        // Generate random fortune
        let mut rng = thread_rng();
        let daikichi = daikichis.choose(&mut rng).unwrap();

        // Display fortune to user
        println!("Your fortune reads: {}", daikichi);
        thread::sleep(Duration::from_secs(1));
        println!("Press enter to generate another fortune, or type 'quit' to exit...");

        // Check for quit command
        let mut command = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        if command.trim() == "quit" {
            break;
        }
    }
}
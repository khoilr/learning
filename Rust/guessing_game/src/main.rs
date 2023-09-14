use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    // Generate a random number between 1 and 100
    let secret = rand::thread_rng().gen_range(1..=100);

    // Start the game loop, like while true
    loop {
        // Create a new mutable string
        let mut guess = String::new();

        // Print a prompt
        println!("Please input your guess: ");

        // Read a line from stdin and store it in the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Print the guess
        println!("You guessed: {}", guess);

        // Convert the guess to a number, and handle errors
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the guess to the secret number, like if statements
        match guess.cmp(&secret) {
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}

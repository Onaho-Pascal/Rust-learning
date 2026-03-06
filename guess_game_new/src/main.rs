use std::io;

fn main() {
    println!("This is a guessing game!");
    println!("Guess a number!");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("Your guessed number is: {guess}");
}
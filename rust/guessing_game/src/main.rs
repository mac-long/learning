use std::io;

fn main() {
    // println! is a macro that prints text to the console
    // The ! indicates that it's a macro, not a function
    println!("Guess the number!");
    println!("Please input your guess.");

    // let creates a variable
    // mut makes the variable mutable allowing the variable to be changed
    let mut guess = String::new(); // mutable variable and bound to a new, empty String

    // This calls the stdin function from the io library
    // Allowing the user to input data to be stored in the guess variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // This line handles errors

    // This line prints the user's guess to the console using string interpolation
    println!("You guessed: {guess}");
}

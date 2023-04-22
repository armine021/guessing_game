// import statement
// std = standard library, io = input/output
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // let creates a new variable of the specified name "guess"
    // Rust variables are immutable by default, so "mut" makes it mutable
    // :: indicates "new" is an associated function of String
    let mut guess = String::new();

    // calls the stdin function from the io module
    // if it hadn't been use called earlier, could run with std::io::stdin
    io::stdin()
        // calls read_lines method on the standard input handle
        // "&mut guess" argument tells where to store the input
        // "&" indicates a reference to avoid copying data into memory multiple times
        // referenes are immutable by default (like variables), so use &mut to make mutable
        // returns a Result to the handle of "Ok" and the generated value or "Err" and why it failed
        .read_line(&mut guess)
        // could have kept this as one line, but separate for readability
        // if Result is "Ok", the generated value is returned
        // if Result is "Err", expect defines error handling
        // for now, keep error handling simple and just print an error statement
        .expect("Failed to read line");

    // {} placeholder for the value of the variable
    // for multiple placeholders, format is prinln!("x = {} and y = {}", x, y);
    println!("You guessed: {guess}");
}

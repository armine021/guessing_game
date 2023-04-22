// import statement
// std = standard library, io = input/output
use std::io;
// compare values and return variants Less, Greater, or Equal
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // let creates a new variable of the specified name "guess"
    // Rust variables are immutable by default, so "mut" makes it mutable
    // :: indicates "new" is an associated function of String
    let mut guess = String::new();

    // The Book uses rand library, but fastrand is smaller and faster
    // creates immutable random number between 1 and 100, inclusive on lower and upper bounds
    let secret_number = fastrand::u32(1..=100);

    // for initial dev, secret_number will be printed so we can predict behavior better
    println!("The secret number is {secret_number}");


    // calls the stdin function from the io module
    // if it hadn't been use called earlier, could run with std::io::stdin
    io::stdin()
        // calls read_line method on the standard input handle
        // "&mut guess" argument tells where to store the input
        // "&" indicates a reference to avoid copying data into memory multiple times
        // referenes are immutable by default (like variables), so use &mut to make mutable
        // returns a Result variant to the handle of "Ok" and the generated value or "Err" and why it failed
        .read_line(&mut guess)
        // could have kept this as one line, but separate for readability
        // if Result is "Ok", the generated value is returned
        // if Result is "Err", expect defines error handling
        // for now, keep error handling simple and just print an error statement
        .expect("Failed to read line");

    // Rust allows shadowing, so multiple variables of the same name can exist as different types
    let guess: u32 = guess
        // eliminate whitespace at beginning and end
        .trim()
        // convert string to another type
        .parse()
        // error message
        .expect("Please type a number!");

    // {} placeholder for the value of the variable
    // for multiple placeholders, format is prinln!("x = {} and y = {}", x, y);
    println!("You guessed: {guess}");

    // compare guess value and reference value for secret_number
    match guess.cmp(&secret_number) {
        // define what to do for each variant on separate arms
        // each arm inclues pattern to match against (like Less) and what to do if true
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("Too big!"),
        Ordering::Greater => println!("You win!"),
    }
}

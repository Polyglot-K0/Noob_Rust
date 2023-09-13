```Rust

// in order to obtain user input and present output
// the 'io' (input/output) library into scope
// The library comes from the standard library - which is 'std'

// if the type you want isnt in the prelude, you have to bring the type into scope
// This is done explicitly into scope with the 'use' statement
// in Rust - variables are immutable by default

use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input the number.");
    // let allows us to store the user input
    // EXAMPLES of let
    // let foo = 5; <- immutable
    // let mut bar = 5; <- mutbale
    // We need to make it mutable in order to change the value with the user input
    // Calling sring::new, a function that returns a new instance of a string
    // the :: syntax in the ::new line that new an associated function of string type
    // this function, creates a new empty string
    // To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String.
    let mut guess = String::new();

    // Calling the input/output we imported from the standard library
    // In the next line - we ll call the stdin function from the io module
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed {}, guess");
    
}
```
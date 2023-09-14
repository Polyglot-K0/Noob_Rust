```Rust

// in order to obtain user input and present output
// the 'io' (input/output) library into scope
// The library comes from the standard library - which is 'std'

// if the type you want isnt in the prelude, you have to bring the type into scope
// This is done explicitly into scope with the 'use' statement
// in Rust - variables are immutable by default

// we need to import the input/output library in order to use its modules
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

    // the .read_line(&mut guess) - calls the read_line method on the standard input handle
    // We re also passing one argument to read_line: &mut guess
    // The job of read_line is to take whatever the user types into standard input and place that into a string
    //.read_line puts what the user types, into the string. It also returns a value

    // THe & indicates that the argument is a reference
    //  & is very similar to a pointer - this prevents us from needing to copying data into memory multiple times
    // & references are immutable by default

    // .expect() method is a 'Result' type - it is used to handle errors

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed {}, guess");
    
} 
```
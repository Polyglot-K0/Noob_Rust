```Rust

// in order to obtain user input and present output
// the 'io' (input/output) library into scope
// The library comes from the standard library - which is 'std'

// if the type you want isnt in the prelude, you have to bring the type into scope
// This is done explicitly into scope with the 'use' statement
// 

use std::io;

fn main() {
    println!("Guess the number!");
    
    println!("Please input the number.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed {}, guess");
    
}
```
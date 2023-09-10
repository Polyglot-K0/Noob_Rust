# Understanding Cargo
    
    - We are gonna create a new projet using cargo
```shell
$ cargo new calculate_factoral_nums  
    Created binary (application) `calculate_factoral_nums` package
```
    - This will creates a new directory called \calculate_factoral_nums\ 
    - the "Cargo workspace," is where Cargo manages the project-specific dependencies and metadata

```shell
$ cd calculate_factoral_nums 
```

    - Now are code that runs in the cargo can be located in \src\ directpry

```console
$ cd src  
``` 
    - withind the src directory you will find main.rs (please see main.src within editor to view code)

```Rust
// Add dependencies to Cargo.toml
// [dependencies]
// num-bigint = "0.4.2"

use num_bigint::BigInt;
use num_traits::One;

fn factorial(n: u32) -> BigInt {
    let mut result = BigInt::one();

    for i in 1..=n {
        result *= i;
    }

    result
}

fn main() {
    let n = 20;
    let result = factorial(n);

    println!("Factorial of {} is: {}", n, result);
}
```
    - This Rust program calculates the factorial of a number.
    - in order to run this program :> the following dependancies must be added to the 'cargo.toml

```toml
[dependencies]
num-bigint = "0.4.2"
num-traits = "0.2.14"
```

    - Once dependancies have been added to TOML then we can run the followingin our terminal

```shell
$ cargo build 
```

    - 
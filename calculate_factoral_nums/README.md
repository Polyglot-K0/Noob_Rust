# Understanding Cargo
    
    - We are gonna create a new projet using cargo
```Rust
$ cargo new calculate_factoral_nums  
    Created binary (application) `calculate_factoral_nums` package
```
    - This will creates a new directory called \calculate_factoral_nums\ 
    - the "Cargo workspace," is where Cargo manages the project-specific dependencies and metadata

```Rust shell 
$ cd calculate_factoral_nums 
```

    - Now are code that runs in the cargo can be located in \src\ directpry

```Rust shell
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

```Rust toml
[dependencies]
num-bigint = "0.4.2"
num-traits = "0.2.14"
```

    - Once dependancies have been added to TOML then we can run the followingin our terminal

```Rust shell
$ cargo build 
```
    - If everything runs smoothly :> the following will print out in your terminal

```Rust shell
Updating crates.io index
Compiling autocfg v1.1.0
Compiling num-traits v0.2.16
Compiling num-integer v0.1.45
Compiling num-bigint v0.4.4
Compiling calculate_factoral_nums v0.1.0 (/Users/taylor_swifty/fun/Rust/Noob_Rust/calculate_factoral_nums)
Finished dev [unoptimized + debuginfo] target(s) in 3.32s
```

    - This command creates an executable file in target/debug/calculate_factoral_nums
    - You can run the executable with this command:

    
```Rust shell
$ ./target/debug/calculate_factoral_nums 
```

    - Which prints out the following results

```Rust shell
Factorial of 20 is: 2432902008176640000
```

    - If all goes well, "Factorial of 20 is: 2432902008176640000" should print to the terminal. Running cargo build for the first time also causes Cargo to create a new file at the top level: Cargo.lock.
     This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you. If you were to open the cargo.lock file in a text editor, the following would come up.

```Rust
# It is not intended for manual editing.
version = 3

[[package]]
name = "autocfg"
version = "1.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"

[[package]]
name = "calculate_factoral_nums"
version = "0.1.0"
dependencies = [
 "num-bigint",
 "num-traits",
]

[[package]]
name = "num-bigint"
version = "0.4.4"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "608e7659b5c3d7cba262d894801b9ec9d00de989e8a82bd4bef91d08da45cdc0"
dependencies = [
 "autocfg",
 "num-integer",
 "num-traits",
]

[[package]]
name = "num-integer"
version = "0.1.45"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "225d3389fb3509a24c93f5c29eb6bde2586b98d9f016636dff58d7c6f7569cd9"
dependencies = [
 "autocfg",
 "num-traits",
]

[[package]]
name = "num-traits"
version = "0.2.16"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "f30b0abd723be7e2ffca1272140fac1a2f084c77ec3e123c192b66af1ee9e6c2"
dependencies = [
 "autocfg",
] 
```

    - We just built a project with cargo build and ran it with ./target/debug/ calculate_factoral_nums, but we can also use cargo run to compile the code and then run the resulting executable all in one command:

```Rust Shell
$ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.06s
Running `target/debug/calculate_factoral_nums`
Factorial of 20 is: 2432902008176640000
```
    - Notice that this time we didn’t see output indicating that Cargo was compiling hello_cargo. Cargo figured out that the files hadn’t changed, so it just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it, and you would have seen this output:

    - Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```Rust Shell
$ cargo check
    Checking num-traits v0.2.16
    Checking num-integer v0.1.45
    Checking num-bigint v0.4.4
    Checking calculate_factoral_nums v0.1.0 (/Users/taylor_swifty/fun/Rust/Noob_Rust/calculate_factoral_nums)
    Finished dev [unoptimized + debuginfo] target(s) in 1.43s
```
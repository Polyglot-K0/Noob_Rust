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
    


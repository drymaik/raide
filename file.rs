pub fn main() {
    let mut a = 3;
    let mut b = 4;
    for i in 0..a {
        for j in 0..b {
            println!("{},{}", i, j);
        }
    }
    let my_factorial = factorial(20);
    println!("My factorial is: {}", my_factorial);
}

pub fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..n + 1 {
        result *= i;
    }
    result
}

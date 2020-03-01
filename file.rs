pub fn main() {
    let mut a = 3;
    let mut b = 4;
    for i in 0..a {
        for j in 0..b {
            println!("{},{}", i, j);
        }
    }
    let my_number = factorial(20);
    println!("My number is: {}", my_number);
    println!("Happy birthday!");
}

pub fn factorial(n: u64) -> u64 {
    let mut result = 1;
    if n == 0 {
        return n;
    }
    for i in 1..n + 1 {
        result *= i;
    }
    result
}

use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Expected a number. Using 0 by default.");
            0
        }
    };

    let nth_fibonacci: u32 = fib(n);
    println!("The {n}th Fibonacci number is: {nth_fibonacci}");
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

pub mod day1;
pub mod day2;

use std::io::stdin;
fn main() {
    loop {
        let mut input = String::new();
        println!("Choose a Day from {} to {}; 0 exits.", 1, 1);
        let input = match stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => panic!("Unexpected input."),
        }
        .chars()
        .nth(0)
        .unwrap_or('0')
        .to_digit(10)
        .unwrap_or(0);

        match input {
            0 => break,
            1 => println!("{}", day1::trebuchet("actual")),
            _ => todo!(),
        }
    }
}

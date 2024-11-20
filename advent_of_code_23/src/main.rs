pub mod day1;

fn main() {
    let day1_result_example = day1::trebuchet("example");
    println!("{day1_result_example}");
    let day1_result = day1::trebuchet("actual");
    println!("{day1_result}");
}

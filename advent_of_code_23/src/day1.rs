// pub mod day1
/*
DAY 1
The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

//use std::{collections::HashSet, fs};
//
//const NUMBER_NAMES: HashSet<&str> = HashSet::from([
//    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
//]);
//
//fn extract_digits_v2(strs: Vec<String>) -> u32 {
//    let mut nums: Vec<u32> = vec![];
//
//    for s in strs {
//        println!("{s}");
//
//        let mut chars = s.chars();
//        let mut chars_rev = s.chars().rev();
//
//        let first_digit = loop {
//            let c = chars.next().expect("No digits were found.");
//
//            if c.is_numeric() {
//                break c;
//            }
//        };
//
//        let last_digit = loop {
//            let c = chars_rev.next().expect("No digits were found.");
//
//            if c.is_numeric() {
//                break c;
//            }
//        };
//
//        let number: u32 = String::from_iter([first_digit, last_digit])
//            .parse()
//            .unwrap();
//        nums.push(number);
//    }
//
//    nums.iter().sum::<u32>()
//}

fn extract_digits(strs: Vec<String>) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for s in strs {
        println!("{s}");

        let mut chars = s.chars();
        let mut chars_rev = s.chars().rev();

        let first_digit = loop {
            let c = chars.next().expect("No digits were found.");

            if c.is_numeric() {
                break c;
            }
        };

        let last_digit = loop {
            let c = chars_rev.next().expect("No digits were found.");

            if c.is_numeric() {
                break c;
            }
        };

        let number: u32 = String::from_iter([first_digit, last_digit])
            .parse()
            .unwrap();
        nums.push(number);
    }

    nums.iter().sum::<u32>()
}

fn parse_text(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).unwrap();
    contents.split_whitespace().map(str::to_string).collect()
}

pub fn trebuchet(s: &str) -> u32 {
    match s {
        "example" => {
            let strs = parse_text(
                "/Users/porlando/Projects/rust/advent_of_code_23/tests/day1_example.txt",
            );
            extract_digits(strs)
        }
        "actual" => {
            let strs = parse_text("/Users/porlando/Projects/rust/advent_of_code_23/tests/day1.txt");
            extract_digits(strs)
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(trebuchet("example"), 142)
    }

    #[test]
    fn test_actual() {
        assert_eq!(trebuchet("actual"), 55447)
    }

    #[test]
    fn test_parse_text() {
        let fpath: &str = "/Users/porlando/Projects/rust/advent_of_code_23/tests/day1_example.txt";
        let expected: [String; 4] =
            ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].map(String::from);
        assert_eq!(parse_text(fpath), expected)
    }

    #[test]
    fn test_extract_digits() {
        let input_strs: Vec<String> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .map(String::from)
            .to_vec();
        let expected: u32 = 142;
        assert_eq!(extract_digits(input_strs), expected)
    }
}

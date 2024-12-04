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

use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::OnceLock;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUMBERS_REV: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

// fn number_set() -> &'static HashSet<&'static str> {
//     static HASHMAP: OnceLock<HashSet<&'static str>> = OnceLock::new();
//     HASHMAP.get_or_init(|| HashSet::from_iter(NUMBERS))
//     // HASHMAP.get_or_init(|| HashSet::from_iter(NUMBERS.map(|c| c.to_string().chars().rev())))
// }

// {"zero", "one", "two", ...}
macro_rules! make_number_set {
    ( $func_name:ident, $x:expr ) => {
        fn $func_name() -> &'static HashSet<&'static str> {
            static HASHMAP: OnceLock<HashSet<&'static str>> = OnceLock::new();
            HASHMAP.get_or_init(|| HashSet::from_iter($x))
        }
    };
}
// {"zero":'0', "one": '1', ...}
macro_rules! make_number_map {
    ( $func_name:ident, $x:expr ) => {
        fn $func_name() -> &'static HashMap<&'static str, char> {
            static HASHMAP: OnceLock<HashMap<&'static str, char>> = OnceLock::new();
            HASHMAP.get_or_init(|| {
                let number_map = $x
                    //.map(|s| s.to_string())
                    .into_iter()
                    .zip((0..$x.len()).map(|i| char::from_digit(i as u32, 10).unwrap()));

                HashMap::from_iter(number_map)
            })
        }
    };
}

// {"o","on","one","t","tw","two","t","th", ...}
macro_rules! make_set_incremental_str {
    ( $func_name:ident, $x:expr ) => {
        fn $func_name() -> &'static HashSet<&'static str> {
            static HASHMAP: OnceLock<HashSet<&str>> = OnceLock::new();
            HASHMAP.get_or_init(|| {
                let number_str_sequential_letters = $x
                    .iter()
                    .flat_map(|&s| (1..=s.len()).map(|i| &s[0..i]).collect::<Vec<&str>>());
                HashSet::from_iter(number_str_sequential_letters)
            })
        }
    };
}

// {'z', 'o', 't', 't', 'f', 'f', 's', 's', 'e', 'n'}
macro_rules! make_first_number_char {
    ( $func_name:ident, $x:expr ) => {
        fn $func_name() -> &'static HashSet<char> {
            static HASHMAP: OnceLock<HashSet<char>> = OnceLock::new();
            HASHMAP.get_or_init(|| {
                let number_str_first_letter_char = $x.map(|s| s.chars().next().unwrap());
                HashSet::from_iter(number_str_first_letter_char)
            })
        }
    };
}

make_number_set!(number_set, NUMBERS);
make_number_set!(number_set_rev, NUMBERS_REV);

make_number_map!(number_map, NUMBERS);
make_number_map!(number_map_rev, NUMBERS_REV);

make_set_incremental_str!(number_set_incremental_str, NUMBERS);
make_set_incremental_str!(number_set_incremental_str_rev, NUMBERS_REV);

make_first_number_char!(number_first_char_mapping, NUMBERS);
make_first_number_char!(number_first_char_mapping_rev, NUMBERS_REV);

// eight1234eight == 88
// oneight == 11
fn extract_digits_v2(strs: Vec<String>) -> u32 {
    let mut all_nums: Vec<u32> = vec![];

    for s in strs {
        let mut nums: Vec<char> = vec![];
        let mut buf: String = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                nums.push(c);
            } else if {
                let extended_str = format!("{}{}", buf, c);
                // println!("{}", extended_str);
                number_set_incremental_str().contains(&extended_str[..])
            } {
                buf.push(c);
                if number_set().contains(&buf[..]) {
                    nums.push(*number_map().get(&buf[..]).unwrap());
                }
            } else {
                buf.clear();
                buf.push(c);
            }
        }

        let first_digit = nums.first().unwrap();
        let last_digit = nums.last().unwrap();
        let number: u32 = String::from_iter([first_digit, last_digit])
            .parse()
            .unwrap();
        all_nums.push(number);
    }

    // println!("{:?}", all_nums);
    all_nums.iter().sum::<u32>()
}

// eight1234eight == 88
// oneight == 18
fn extract_digits_v3(strs: Vec<String>) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for s in strs {
        let mut buf = String::new();
        let mut chars = s.chars();
        let mut chars_rev = s.chars().rev();

        let first_digit = loop {
            let c = chars.next().expect("No digits were found.");

            if c.is_numeric() {
                break c;
            } else if {
                let extended_str = format!("{}{}", buf, c);
                number_set_incremental_str().contains(&extended_str[..])
            } {
                buf.push(c);
                if number_set().contains(&buf[..]) {
                    break *number_map().get(&buf[..]).unwrap();
                }
            } else if number_first_char_mapping().contains(&c) {
                buf.clear();
                buf.push(c);
            }
        };

        let last_digit = loop {
            let c = chars_rev.next().expect("No digits were found.");

            if c.is_numeric() {
                break c;
            } else if {
                let extended_str = format!("{}{}", buf, c);
                number_set_incremental_str_rev().contains(&extended_str[..])
            } {
                buf.push(c);
                if number_set_rev().contains(&buf[..]) {
                    break *number_map_rev().get(&buf[..]).unwrap();
                }
            } else if number_first_char_mapping_rev().contains(&c) {
                buf.clear();
                buf.push(c);
            }
        };

        let number: u32 = String::from_iter([first_digit, last_digit])
            .parse()
            .unwrap();
        nums.push(number);
    }

    //println!("{:?}", nums);
    nums.iter().sum::<u32>()
}

// eight1234eight == 14
fn extract_digits(strs: Vec<String>) -> u32 {
    let mut nums: Vec<u32> = vec![];

    for s in strs {
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
        "example2" => {
            let strs = parse_text(
                "/Users/porlando/Projects/rust/advent_of_code_23/tests/day1_example2.txt",
            );
            extract_digits_v2(strs)
        }
        "example3" => {
            let strs = parse_text(
                "/Users/porlando/Projects/rust/advent_of_code_23/tests/day1_example2.txt",
            );
            extract_digits_v3(strs)
        }
        "actual" => {
            let strs = parse_text("/Users/porlando/Projects/rust/advent_of_code_23/tests/day1.txt");
            extract_digits(strs)
        }
        "actual2" => {
            let strs = parse_text("/Users/porlando/Projects/rust/advent_of_code_23/tests/day1.txt");
            extract_digits_v2(strs)
        }
        "actual3" => {
            let strs = parse_text("/Users/porlando/Projects/rust/advent_of_code_23/tests/day1.txt");
            extract_digits_v3(strs)
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
    fn test_v2() {
        assert_eq!(trebuchet("example2"), 281);
    }

    #[test]
    fn test_v3() {
        assert_eq!(trebuchet("example3"), 281);
    }

    #[test]
    fn test_actual() {
        assert_eq!(trebuchet("actual"), 55447);
    }

    #[test]
    fn test_actual_v2() {
        assert!(trebuchet("actual2") > 54569);
        assert!(trebuchet("actual2") < 54729);
        assert_ne!(trebuchet("actual2"), 54670);
    }

    #[test]
    fn test_actual_v3() {
        assert!(trebuchet("actual3") > 54569);
        assert!(trebuchet("actual3") < 54729);
        assert_ne!(trebuchet("actual3"), 54670);
    }

    #[test]
    fn test_extract_digits() {
        let input_strs: Vec<String> = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .map(String::from)
            .to_vec();
        let expected: u32 = 142;
        assert_eq!(extract_digits(input_strs), expected)
    }

    #[test]
    fn test_parse_text() {
        let fpath: &str = "/Users/porlando/Projects/rust/advent_of_code_23/tests/day1_example.txt";
        let expected: [String; 4] =
            ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].map(String::from);
        assert_eq!(parse_text(fpath), expected)
    }
}

/* Plan of action:
*   a matrix (vec of vecs in Rust?)
*   every time we find a symbol (non-alphanumeric character, also not a .),
*       get all adjacent numbers and add to Vec<i32> (?)
*           > possible edge case: what if we have a number adjacent to two different symbols?
*   return sum of Vec<i32>
*/

mod schematic_processor {
    use std::fs;

    fn build_matrix(filename: &str) -> Vec<Vec<char>> {
        let contents = fs::read_to_string(filename).unwrap();
        Vec::from_iter(contents.lines().map(|l| l.chars().collect()))
    }

    pub fn process(filename: &str) -> i32 {
        // let m = build_matrix(filename);
        4361
    }
}

pub fn decipher_schematic(s: &str) -> i32 {
    match s {
        "example" => schematic_processor::process("./tests/day3_example.txt"),
        "example_v2" => todo!(),
        "actual" => todo!(),
        "actual_v2" => todo!(),
        _ => todo!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(decipher_schematic("example"), 4361);
    }
}

/* Plan of action:
*   a matrix (vec of vecs in Rust?)
*   every time we find a symbol (non-alphanumeric character, also not a .),
*       get all adjacent numbers and add to Vec<i32> (?)
*           > possible edge case: what if we have a number adjacent to two different symbols?
*   return sum of Vec<i32>
*/

mod schematic_processor {
    use std::fs;

    pub trait SpecialCharacter {
        fn is_special_character(c: char) -> bool;
    }

    impl SpecialCharacter for char {
        fn is_special_character(c: char) -> bool {
            c != '.' && !c.is_alphanumeric()
        }
    }
    //  Input: filename
    //  Return example:
    //      Vec[
    //      [ 467..114.. ]
    //      [ ...*...... ]
    //      [ ..35..633. ]
    //      [ ......#... ]
    //      [ 617*...... ]
    //      [ .....+.58. ]
    //      [ ..592..... ]
    //      [ ......755. ]
    //      [ ...$.*.... ]
    //      [ .664.598.. ]
    //      ]
    //  Where all of these are chars
    fn build_matrix(filename: &str) -> Vec<Vec<char>> {
        let contents = fs::read_to_string(filename).unwrap();
        Vec::from_iter(contents.lines().map(|l| l.chars().collect()))
    }

    pub fn process(filename: &str) -> i32 {
        let m = build_matrix(filename);
        for i in 0..m.len() {
            println!("{:?}", m[i]);
            for j in 0..m[i].len() {
                if char::is_special_character(m[i][j]) {
                    // check surroundings, whole schbang about not going out of bounds etc
                    // if a digit is found, magic stuff will happen
                    //      > how do check the full digit, cant repeat it
                    //      > idea: skip if middle of number?
                    //      > better idea: keep a different matrix keeping track of saved digits
                    //      >           e.g. 4 6 7        1 1 1
                    //                       . # .   =>   0 ? 0
                    //                       1 4 6        0 0 0
                    //                  after finding 467 it's marked as 1
                }
            }
        }
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

    #[test]
    fn is_special_character() {
        // Supposed to be a special character
        assert!(<char as schematic_processor::SpecialCharacter>::is_special_character('@'));
        assert!(<char as schematic_processor::SpecialCharacter>::is_special_character('#'));
        assert!(<char as schematic_processor::SpecialCharacter>::is_special_character('*'));

        // Not supposed to be a special character
        assert!(!<char as schematic_processor::SpecialCharacter>::is_special_character('a'));
        assert!(!<char as schematic_processor::SpecialCharacter>::is_special_character('1'));
        assert!(!<char as schematic_processor::SpecialCharacter>::is_special_character('.'));
    }
}

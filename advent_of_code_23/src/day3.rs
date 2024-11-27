/* Plan of action:
*   a matrix (vec of vecs in Rust?)
*   every time we find a symbol (non-alphanumeric character, also not a .),
*       get all adjacent numbers and add to Vec<u32> (?)
*           > possible edge case: what if we have a number adjacent to two different symbols?
*   return sum of Vec<u32>
*/

mod schematic_processor {
    use std::{
        cmp::{max, min},
        fs,
    };

    pub trait SpecialCharacter {
        fn is_special_character(c: char) -> bool;
    }

    impl SpecialCharacter for char {
        fn is_special_character(c: char) -> bool {
            c != '.' && !c.is_alphanumeric()
        }
    }

    fn get_locations(i: usize, j: usize, max_i: &usize, max_j: &usize) -> Vec<(usize, usize)> {
        let min_i = max(0, i - 1);
        let min_j = max(0, j - 1);

        let max_i = min(*max_i - 1, i + 1);
        let max_j = min(*max_j - 1, j + 1);

        let mut locations = vec![];
        for i in min_i..=max_i {
            for j in min_j..=max_j {
                if i == 0 && j == 0 {
                    continue;
                }
                locations.push((i, j));
            }
        }

        locations
    }

    fn get_number(
        i: usize,
        j: usize,
        max_j: &usize,
        m: &Vec<Vec<char>>,
        mc: &mut Vec<Vec<u32>>,
    ) -> u32 {
        let mut k = j;
        while k > 0 && m[i][k - 1].is_digit(10) {
            k -= 1;
        }

        let mut buf = "".to_string();

        while k < *max_j && m[i][k].is_digit(10) {
            buf.push(m[i][k]);
            (*mc)[i][k] = 1;
            k += 1;
        }

        buf.parse::<u32>().unwrap()
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

    fn build_check_matrix(m: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
        let mc = m
            .clone()
            .iter()
            .map(|l| l.iter().map(|_| 0).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();
        mc
    }

    pub fn process(filename: &str) -> u32 {
        let m = build_matrix(filename);
        let mut mc = build_check_matrix(&m);
        let mut cumulative_sum = 0;

        for i in 0..m.len() {
            for j in 0..m[i].len() {
                if char::is_special_character(m[i][j]) {
                    // check surroundings, whole schbang about not going out of bounds etc
                    let locations = get_locations(i, j, &m.len(), &m[i].len());

                    for (k_i, k_j) in locations {
                        // if a digit is found, magic stuff will happen
                        if m[k_i][k_j].is_digit(10) && mc[k_i][k_j] == 0 {
                            // keep a different matrix keeping track of saved digits
                            //  e.g.    4 6 7        1 1 1
                            //          . # .   =>   0 0 0
                            //          1 4 6        0 0 0
                            //      after finding 467 it's marked as 1
                            cumulative_sum += get_number(k_i, k_j, &m[i].len(), &m, &mut mc);
                        }
                    }
                }
            }
        }
        for i in 0..m.len() {
            let mcs: String = mc[i]
                .clone()
                .into_iter()
                .map(|i| match i {
                    0 => ".".to_string(),
                    i => i.to_string(),
                })
                .collect::<String>();
            println!("{mcs}");
            // println!("{:?}", mc[i]);
        }
        println!("{cumulative_sum}");
        cumulative_sum
    }
}

pub fn decipher_schematic(s: &str) -> u32 {
    match s {
        "example" => schematic_processor::process("./tests/day3_example.txt"),
        "actual" => schematic_processor::process("./tests/day3.txt"),
        _ => todo!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(decipher_schematic("example"), 4372);
    }

    #[test]
    fn test_actual() {
        assert_eq!(decipher_schematic("actual"), 539637);
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

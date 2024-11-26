use std::fs::read_to_string;
enum Colors {
    Red,
    Green,
    Blue,
}
const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//  ["Game 1", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
//      3 blue, 4 red
//          ["3 blue", "4 red"]
//          [["3", "blue"], ["4", "red"]]
//          match ...
//              (4,0,3)
//      1 red, 2 green, 6 blue
//      2 green
//
//  Ret:
//      (1, Vec["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"])
//      (1, Vec[(4, 0, 3), (1, 2, 6), (0, 2, 0)])
fn parse_game_line(line: &str) -> (i32, Vec<(i32, i32, i32)>) {
    let ls: Vec<&str> = line.split(": ").collect();
    let (game, rounds) = (ls[0], ls[1]);

    // Getting the Game ID
    let game_id = game.split(" ").last().unwrap().parse::<i32>().unwrap();

    // Parsing game rounds into turns
    // rounds: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // turns: ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
    let turns: Vec<&str> = rounds.split("; ").collect();

    // turns: [[(3, BLUE), (4, RED)], [(1, RED), (2, GREEN), (6, BLUE)], [(2, GREEN)]]
    let turns: Vec<Vec<(i32, Colors)>> = turns
        .iter()
        .map(|s| {
            s.split(", ")
                .map(|s| {
                    // ss: ["3", "blue"]
                    let mut ss = s.split(" ");

                    // d: 3
                    let d = ss.next().unwrap().parse::<i32>().unwrap();

                    // color: Colors::Blue
                    let color = match ss.next().unwrap() {
                        "red" => Colors::Red,
                        "green" => Colors::Green,
                        "blue" => Colors::Blue,
                        _ => panic!("Unknown colour"),
                    };
                    (d, color)
                })
                .collect()
        })
        .collect();

    let mut result: Vec<(i32, i32, i32)> = vec![];

    for turn in turns {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for (n, c) in turn {
            match c {
                Colors::Red => r = n,
                Colors::Green => g = n,
                Colors::Blue => b = n,
            };
        }
        result.push((r, g, b))
    }

    (game_id, result)
}

fn play_cube_conundrum(filename: &str) -> i32 {
    let mut games = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let result = parse_game_line(line);
        if result
            .1
            .iter()
            .filter(|(r, g, b)| *r > MAX_RED || *g > MAX_GREEN || *b > MAX_BLUE)
            .count()
            == 0
        {
            //println!("{:?}", result);
            games.push(result.0);
        }
    }
    //println!("{:?}", games);
    games.iter().sum()
}

fn play_cube_conundrum_v2(filename: &str) -> i32 {
    let mut games = vec![];
    for line in read_to_string(filename).unwrap().lines() {
        let result = parse_game_line(line);
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for (r, g, b) in result.1 {
            if r > max_red {
                max_red = r;
            }
            if g > max_green {
                max_green = g;
            }
            if b > max_blue {
                max_blue = b;
            }
        }
        games.push(max_red * max_green * max_blue);
    }
    //println!("{:?}", games);
    games.iter().sum()
}

pub fn cube_conundrum(s: &str) -> i32 {
    match s {
        "example" => play_cube_conundrum("./tests/day2_example.txt"),
        "example_v2" => play_cube_conundrum_v2("./tests/day2_example.txt"),
        "actual" => play_cube_conundrum("./tests/day2.txt"),
        "actual_v2" => play_cube_conundrum_v2("./tests/day2.txt"),
        _ => todo!(),
    }
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(cube_conundrum("example"), 8);
    }

    #[test]
    fn test_example_v2() {
        assert_eq!(cube_conundrum("example_v2"), 2286);
    }

    #[test]
    fn test_actual() {
        assert_eq!(cube_conundrum("actual"), 2076);
    }

    #[test]
    fn test_actual_v2() {
        assert_eq!(cube_conundrum("actual_v2"), 70950);
    }
}

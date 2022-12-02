use std::str::FromStr;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

const MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

fn parse_move(input: &str) -> i32 {
    match input {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("Invalid move"),
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut moves = line.split_ascii_whitespace().take(2).map(parse_move);
            let first = moves.next().expect("Valid Move");
            let second = moves.next().expect("Valid Move");

            (second + 1)
                + if (first + 1) % 3 == second {
                    6
                } else if first == second {
                    3
                } else {
                    0
                }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut moves = line.split_ascii_whitespace().take(2).map(parse_move);
            let first = moves.next().expect("Valid Move");
            let second = moves.next().expect("Valid Move");

            (second * 3) + (((second - 1) + first) % 3) + 1
        })
        .sum()
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day2.txt").expect("Need input");

    let score = part1(&input);
    println!("Part 1 Score: {}", score);
    let score = part2(&input);
    println!("Part 2 Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "A Y
B X
C Z";

        assert_eq!(15, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "A Y
B X
C Z";

        assert_eq!(12, part2(input));
    }
}

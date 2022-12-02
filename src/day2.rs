use std::str::FromStr;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

type Round = (Move, Move);

fn move_beats_move(m1: Move, m2: Move) -> bool {
    match m1 {
        Move::Rock => m2 == Move::Scissors,
        Move::Paper => m2 == Move::Rock,
        Move::Scissors => m2 == Move::Paper,
    }
}

fn score_round(round: Round) -> u32 {
    round.1.score()
        + if move_beats_move(round.1, round.0) {
            6
        } else if round.0 == round.1 {
            3
        } else {
            0
        }
}

fn parse_round(input: &str) -> Round {
    let mut moves = input.split_ascii_whitespace().take(2);
    let first = moves.next().expect("Should be a valid move");
    let second = moves.next().expect("Should be a valid move");

    let first = parse_move(first);
    let second = parse_move(second);

    (first, second)
}

fn parse_move(input: &str) -> Move {
    match input {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => panic!("Invalid move"),
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

fn parse_strategy(input: &str) -> Strategy {
    match input {
        "X" => Strategy::Lose,
        "Y" => Strategy::Draw,
        "Z" => Strategy::Win,
        _ => panic!("Invalid strategy"),
    }
}

fn fit_strategy(mov: Move, strat: Strategy) -> Move {
    match mov {
        Move::Rock => match strat {
            Strategy::Lose => Move::Scissors,
            Strategy::Draw => Move::Rock,
            Strategy::Win => Move::Paper,
        },
        Move::Paper => match strat {
            Strategy::Lose => Move::Rock,
            Strategy::Draw => Move::Paper,
            Strategy::Win => Move::Scissors,
        },
        Move::Scissors => match strat {
            Strategy::Lose => Move::Paper,
            Strategy::Draw => Move::Scissors,
            Strategy::Win => Move::Rock,
        },
    }
}

fn parse_part_two(input: &str) -> (Move, Strategy) {
    let mut moves = input.split_ascii_whitespace().take(2);
    let first = moves.next().expect("Should be a valid move");
    let second = moves.next().expect("Should be a valid strategy");

    let first = parse_move(first);
    let second = parse_strategy(second);

    (first, second)
}

fn part1(input: &str) {
    let score: u32 = input.lines().map(parse_round).map(score_round).sum();

    println!("Score: {}", score)
}

fn part2(input: &str) {
    let score: u32 = input
        .lines()
        .map(parse_part_two)
        .map(|(m, s)| (m, fit_strategy(m, s)))
        .map(score_round)
        .sum();

    println!("Part 2 Score: {}", score);
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day2.txt").expect("Need input");

    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "A Y
B X
C Z";

        let rounds: Vec<Round> = input.lines().map(parse_round).collect();

        assert_eq!(
            vec![
                (Move::Rock, Move::Paper),
                (Move::Paper, Move::Rock),
                (Move::Scissors, Move::Scissors)
            ],
            rounds
        );

        let first = score_round(rounds[0]);
        assert_eq!(8, first);

        let second = score_round(rounds[1]);
        assert_eq!(1, second);

        let third = score_round(rounds[2]);
        assert_eq!(6, third);

        assert_eq!(15, first + second + third);
    }

    #[test]
    fn parse_part2() {
        let input = "A Y
B X
C Z";

        let moves: Vec<(Move, Strategy)> = input.lines().map(parse_part_two).collect();
        assert_eq!(
            vec![
                (Move::Rock, Strategy::Draw),
                (Move::Paper, Strategy::Lose),
                (Move::Scissors, Strategy::Win)
            ],
            moves
        );
    }

    #[test]
    fn part2() {
        let input = "A Y
B X
C Z";
        let score: u32 = input
            .lines()
            .map(parse_part_two)
            .map(|(m, s)| (m, fit_strategy(m, s)))
            .map(score_round)
            .sum();

        assert_eq!(12, score);
    }
}

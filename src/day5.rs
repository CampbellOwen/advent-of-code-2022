use std::str::FromStr;

use anyhow::{bail, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Instruction {
    num: u32,
    from: u32,
    to: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let captures = RE.captures_iter(s).next().ok_or("No matches")?;

        let num = captures[1].parse().map_err(|_| "Not a num")?;
        let from = captures[2].parse().map_err(|_| "Not a num")?;
        let to = captures[3].parse().map_err(|_| "Not a num")?;
        Ok(Instruction { num, from, to })
    }
}

struct Board {
    num: u32,
    stacks: Vec<Vec<String>>,
}

impl Board {
    pub fn new(num: u32) -> Board {
        let mut stacks = Vec::new();
        stacks.resize(num as usize, vec![]);
        Board { num, stacks }
    }
    pub fn execute(&mut self, instruction: &Instruction, in_order: bool) -> Result<()> {
        if instruction.from as usize > self.stacks.len()
            || instruction.to as usize > self.stacks.len()
        {
            bail!("Invalid location");
        }

        if in_order {
            let mut temp = vec![];
            for _ in 0..instruction.num {
                let obj = self.stacks[instruction.from as usize - 1]
                    .pop()
                    .context("Not enough items")?;

                temp.push(obj);
            }

            for _ in 0..temp.len() {
                self.stacks[instruction.to as usize - 1]
                    .push(temp.pop().context("Not enough in temp")?);
            }
        } else {
            for _ in 0..instruction.num {
                let obj = self.stacks[instruction.from as usize - 1]
                    .pop()
                    .context("Not enough items")?;

                self.stacks[instruction.to as usize - 1].push(obj);
            }
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let num: u32 = lines
            .last()
            .ok_or("Not enough lines")?
            .split_ascii_whitespace()
            .last()
            .ok_or("Not enough characters")?
            .parse()
            .map_err(|_| "Not a number")?;

        let mut board = Board::new(num);

        for i in (0..lines.len() - 1).rev() {
            for (c_i, x) in (1..(4 * num)).step_by(4).enumerate() {
                let c = lines[i].chars().nth(x as usize).ok_or("Not enough chars")?;
                if !c.is_whitespace() {
                    let s = String::from(c);
                    board.stacks[c_i].push(s);
                }
            }
        }

        Ok(board)
    }
}

fn parse_input(input: &str) -> Result<(Board, Vec<Instruction>)> {
    let mut split = input.split("\n\n");

    let board = split
        .next()
        .context("Malformed input".to_owned())?
        .parse::<Board>()
        .ok()
        .context("Not a board")?;

    let instructions = split
        .next()
        .map(|s| s.lines().filter_map(|line| line.parse().ok()))
        .context("No instructions")?
        .collect::<Vec<Instruction>>();

    Ok((board, instructions))
}

fn part1(input: &str) -> String {
    let (mut board, instructions) = parse_input(input).expect("Valid input");
    for inst in &instructions {
        board.execute(inst, false).expect("Succeeded");
    }

    board
        .stacks
        .iter()
        .map(|stack| stack.last().map_or(String::new(), |s| s.clone()))
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (mut board, instructions) = parse_input(input).expect("Valid input");
    for inst in &instructions {
        board.execute(inst, true).expect("Succeeded");
    }

    board
        .stacks
        .iter()
        .map(|stack| stack.last().map_or(String::new(), |s| s.clone()))
        .collect::<String>()
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day5.txt").expect("Need input");

    let score = part1(&input);
    println!("[Day  5][Part 1] - {}", score);
    let score = part2(&input);
    println!("[Day  5][Part 2] - {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_inst_test() {
        let mut stacks = Board::new(3);
        stacks.stacks[0].push("Z".to_owned());

        let inst = Instruction {
            num: 1,
            from: 1,
            to: 2,
        };

        stacks.execute(&inst, false).unwrap();

        assert_eq!("Z", stacks.stacks[1][0]);
        assert_eq!(0, stacks.stacks[0].len());
    }

    #[test]
    fn parse_board_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

        let board = Board::from_str(input).unwrap();
        assert_eq!(3, board.num);

        assert_eq!("Z", board.stacks[0][0]);
        assert_eq!("N", board.stacks[0][1]);

        assert_eq!("M", board.stacks[1][0]);
        assert_eq!("C", board.stacks[1][1]);
        assert_eq!("D", board.stacks[1][2]);

        assert_eq!("P", board.stacks[2][0]);
    }

    #[test]
    fn parse_instruction_test() {
        let input = "move 1 from 2 to 1";
        let instruction = input.parse().unwrap();
        assert_eq!(
            Instruction {
                num: 1,
                from: 2,
                to: 1
            },
            instruction
        );
    }

    #[test]
    fn parse_input_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (board, instructions) = parse_input(input).unwrap();
        assert_eq!(3, board.num);

        assert_eq!("Z", board.stacks[0][0]);
        assert_eq!("N", board.stacks[0][1]);

        assert_eq!("M", board.stacks[1][0]);
        assert_eq!("C", board.stacks[1][1]);
        assert_eq!("D", board.stacks[1][2]);

        assert_eq!("P", board.stacks[2][0]);

        assert_eq!(4, instructions.len());
        assert_eq!(
            Instruction {
                num: 1,
                from: 2,
                to: 1
            },
            instructions[0]
        );
        assert_eq!(
            Instruction {
                num: 3,
                from: 1,
                to: 3
            },
            instructions[1]
        );
        assert_eq!(
            Instruction {
                num: 2,
                from: 2,
                to: 1
            },
            instructions[2]
        );
        assert_eq!(
            Instruction {
                num: 1,
                from: 1,
                to: 2
            },
            instructions[3]
        );
    }

    #[test]
    fn part1_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("CMZ", part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", part2(input));
    }
}

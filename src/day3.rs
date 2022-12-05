use itertools::Itertools;
use std::{borrow::BorrowMut, collections::HashSet};

#[derive(Debug, PartialEq)]
struct Rucksack<'a> {
    first: &'a str,
    first_hash: HashSet<char>,
    second: &'a str,
    second_hash: HashSet<char>,
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        if s.len() % 2 != 0 {
            panic!("Compartments different sizes");
        }

        let first = &s[0..(s.len() / 2)];
        let second = &s[(s.len() / 2..)];
        Rucksack {
            first,
            first_hash: HashSet::from_iter(first.chars()),
            second,
            second_hash: HashSet::from_iter(second.chars()),
        }
    }
}

impl<'a> Rucksack<'a> {
    pub fn common_item(&self) -> char {
        self.first
            .chars()
            .find(|c| self.second_hash.contains(c))
            .expect("Should be a match")
    }
}

fn char_score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Rucksack::from(line).common_item())
        .map(char_score)
        .sum()
}

struct Group<'a>(Rucksack<'a>, Rucksack<'a>, Rucksack<'a>);

impl<'a> From<&'a str> for Group<'a> {
    fn from(s: &'a str) -> Self {
        let mut sacks = s.lines().take(3).map(Rucksack::from);

        Group(
            sacks.next().expect("Not enough sacks"),
            sacks.next().expect("Not enough sacks"),
            sacks.next().expect("Not enough sacks"),
        )
    }
}

impl<'a> Group<'a> {
    pub fn common_item(&self) -> char {
        self.0
            .first
            .chars()
            .find(|c| {
                (self.1.first_hash.contains(c) || self.1.second_hash.contains(c))
                    && (self.2.first_hash.contains(c) || self.2.second_hash.contains(c))
            })
            .unwrap_or_else(|| {
                self.0
                    .second
                    .chars()
                    .find(|c| {
                        (self.1.first_hash.contains(c) || self.1.second_hash.contains(c))
                            && (self.2.first_hash.contains(c) || self.2.second_hash.contains(c))
                    })
                    .expect("No match")
            })
    }
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .borrow_mut()
        .map(|mut c| c.join("\n"))
        .map(|chunk| {
            let s = &chunk as &str;
            Group::from(s).common_item()
        })
        .map(char_score)
        .sum()
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day3.txt").expect("Need input");

    let score = part1(&input);
    println!("Part 1 Score: {}", score);
    let score = part2(&input);
    println!("Part 2 Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rucksack_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from(input);
        assert_eq!("vJrwpWtwJgWr", rucksack.first);
        assert_eq!("hcsFMMfFFhFp", rucksack.second);
    }

    #[test]
    fn common_item_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from(input);

        assert_eq!('p', rucksack.common_item());
    }

    #[test]
    fn char_score_test() {
        assert_eq!(1, char_score('a'));
        assert_eq!(26, char_score('z'));
        assert_eq!(27, char_score('A'));
        assert_eq!(52, char_score('Z'));
    }

    #[test]
    fn part1_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, part1(input));
    }

    #[test]
    fn matching_group_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";

        let group = Group::from(input);
        let common = group.common_item();
        assert_eq!('r', common);
    }

    #[test]
    fn part2_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let score = part2(input);
        assert_eq!(70, score);
    }
}

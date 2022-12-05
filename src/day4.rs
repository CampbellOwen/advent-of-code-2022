use num::Num;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Interval<T: Debug + Num + PartialEq + PartialOrd> {
    min: T,
    max: T,
}
impl<T: Debug + Num + PartialEq + PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.min.partial_cmp(&other.min) {
            Some(Equal) => {}
            ord => return ord,
        }
        self.max.partial_cmp(&other.max)
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }
}

impl<T: Debug + Num + PartialEq + PartialOrd> Interval<T> {
    pub fn contains(&self, other: &Interval<T>) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn overlaps(&self, other: &Interval<T>) -> bool {
        self.max >= other.min && self.min <= other.max
    }
}

impl<T: Debug + Num + PartialEq + PartialOrd + FromStr> FromStr for Interval<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split('-').take(2).filter_map(|num| num.parse().ok());

        let min = nums.next().ok_or("Not enough numbers")?;
        let max = nums.next().ok_or("Not enough numbers")?;

        Ok(Interval { min, max })
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut intervals = s.split(',').filter_map(|s| s.parse().ok());
            let first: Interval<u32> = intervals.next().expect("Valid interval");
            let second: Interval<u32> = intervals.next().expect("Valid interval");

            first.contains(&second) || second.contains(&first)
        })
        .filter(|&p| p)
        .count() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut intervals = s.split(',').filter_map(|s| s.parse().ok());
            let first: Interval<u32> = intervals.next().expect("Valid interval");
            let second: Interval<u32> = intervals.next().expect("Valid interval");

            first.overlaps(&second) || second.contains(&first)
        })
        .filter(|&p| p)
        .count() as u32
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day4.txt").expect("Need input");

    let score = part1(&input);
    println!("[Day  4][Part 1] - {}", score);
    let score = part2(&input);
    println!("[Day  4][Part 2] - {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_ordering() {
        let a = Interval { min: 5, max: 10 };
        let b = Interval { min: 6, max: 9 };

        assert!(b.gt(&a));
        assert!(a.lt(&b));

        let a = Interval { min: 6, max: 10 };
        assert!(a.gt(&b));
    }

    #[test]
    fn interval_contains_test() {
        let a = Interval { min: 5, max: 10 };
        let b = Interval { min: 6, max: 9 };

        assert!(a.contains(&b));
        assert!(!b.contains(&a));

        let b = Interval { min: 5, max: 10 };
        assert!(a.contains(&b));
        assert!(b.contains(&a));
    }

    #[test]
    fn interval_overlap_test() {
        let a: Interval<u32> = "5-7".parse().unwrap();
        let b: Interval<u32> = "7-9".parse().unwrap();
        assert!(a.overlaps(&b));

        let a: Interval<u32> = "2-8".parse().unwrap();
        let b: Interval<u32> = "3-7".parse().unwrap();
        assert!(a.overlaps(&b));

        let a: Interval<u32> = "6-6".parse().unwrap();
        let b: Interval<u32> = "4-6".parse().unwrap();
        assert!(a.overlaps(&b));

        let a: Interval<u32> = "2-6".parse().unwrap();
        let b: Interval<u32> = "4-8".parse().unwrap();
        assert!(a.overlaps(&b));

        let a: Interval<u32> = "2-4".parse().unwrap();
        let b: Interval<u32> = "6-8".parse().unwrap();
        assert!(!a.overlaps(&b));

        let a: Interval<u32> = "2-3".parse().unwrap();
        let b: Interval<u32> = "4-5".parse().unwrap();
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn parse_test() {
        let input = "2-4";
        assert_eq!(Interval::<u32> { min: 2, max: 4 }, input.parse().unwrap())
    }

    #[test]
    fn part1_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(4, part2(input));
    }
}

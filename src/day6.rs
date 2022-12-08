use itertools::Itertools;

fn part1(input: &str) -> u32 {
    let mut unique_since = 0;

    for (i, c) in input.chars().enumerate() {
        if i - unique_since == 4 {
            return i as u32;
        }
        if let Some((index, _)) = input[unique_since..i]
            .chars()
            .find_position(|&new_char| c == new_char)
        {
            unique_since += index + 1;
        }
    }

    0
}

fn part2(input: &str) -> u32 {
    let mut unique_since = 0;

    for (i, c) in input.chars().enumerate() {
        if i - unique_since == 14 {
            return i as u32;
        }
        if let Some((index, _)) = input[unique_since..i]
            .chars()
            .find_position(|&new_char| c == new_char)
        {
            unique_since += index + 1;
        }
    }

    0
}

pub fn solve() {
    let input = std::fs::read_to_string("input/day6.txt").expect("Need input");

    let score = part1(&input);
    println!("[Day  6][Part 1] - {}", score);
    let score = part2(&input);
    println!("[Day  6][Part 2] - {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, part1(input));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, part1(input));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, part1(input));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, part1(input));
    }
}

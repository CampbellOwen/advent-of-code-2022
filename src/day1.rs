#[derive(PartialEq, Debug)]
struct Elf {
    calories: u32,
}

fn parse_elves(input: &str) -> Vec<Elf> {
    input
        .split_terminator("\n\n")
        .map(|section| {
            let calories = section
                .lines()
                .map(|line| line.parse::<u32>().unwrap_or(0))
                .sum();
            Elf { calories }
        })
        .collect()
}
fn most_calories(elves: &[Elf]) -> u32 {
    elves
        .iter()
        .max_by_key(|&e| e.calories)
        .map_or(0, |e| e.calories)
}

fn top_three_calories(elves: &mut [Elf]) -> u32 {
    elves.sort_by_key(|e| std::cmp::Reverse(e.calories));

    elves[0..3].iter().map(|e| e.calories).sum()
}

pub fn part1() {
    let input =
        std::fs::read_to_string("input/day1_part1.txt").expect("Need to provide puzzle input");

    let elves = parse_elves(&input);
    let biggest_elf = most_calories(&elves);
    println!("[Day  1][Part 1] - {}", biggest_elf);
}

pub fn part2() {
    let input =
        std::fs::read_to_string("input/day1_part1.txt").expect("Need to provide puzzle input");

    let mut elves = parse_elves(&input);
    let top_3 = top_three_calories(&mut elves);
    println!("[Day  1][Part 2] - {}", top_3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let elves = parse_elves(input);
        assert_eq!(
            vec![
                Elf { calories: 6000 },
                Elf { calories: 4000 },
                Elf { calories: 11000 },
                Elf { calories: 24000 },
                Elf { calories: 10000 }
            ],
            elves
        )
    }

    #[test]
    fn part1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let elves = parse_elves(input);
        let max_calories = most_calories(&elves);
        assert_eq!(24000, max_calories);
    }

    #[test]
    fn part2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let mut elves = parse_elves(input);
        let max_calories = top_three_calories(&mut elves);
        assert_eq!(45000, max_calories);
    }
}

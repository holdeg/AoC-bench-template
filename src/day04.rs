use std::collections::{HashSet, VecDeque};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day04;

impl Solution for Day04 {
    type ParsedInput = Vec<HashSet<u32>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.

        input_lines
            .lines()
            .map(|line| line.split_once(":").unwrap().1)
            .map(|line| line.split_once("|").unwrap().into())
            .map(|sets: [&str; 2]| {
                let lists = sets.map(|set| {
                    set.split_whitespace()
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<HashSet<_>>()
                });
                lists[0]
                    .intersection(&lists[1])
                    .copied()
                    .collect::<HashSet<_>>()
            })
            .collect()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input
            .iter()
            .fold(0, |acc, intersections| {
                acc + intersections
                    .len()
                    .checked_sub(1)
                    .map(|pow| 2u32.pow(pow as u32))
                    .unwrap_or(0)
            })
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut total = 0u32;
        let mut modifiers = [1u32; 11].into_iter().collect::<VecDeque<_>>();
        for item in input {
            for idx in 0..item.len() {
                modifiers[idx + 1] += modifiers[0]
            }
            total += modifiers[0];
            modifiers.rotate_left(1);
            modifiers[10] = 1;
        }
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            Day04::solve_part_one(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "13".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(
            Day04::solve_part_two(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            "30".to_string()
        )
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(Day04::solve("", false), ("0".to_string(), "0".to_string()))
    }
}

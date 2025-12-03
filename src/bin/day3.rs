use itertools::Itertools;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(357, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(3121910778619, r);
    }
}

fn find_joltage(digits: &[usize]) -> usize {
    let digits1 = &digits[0..digits.len() - 1];
    let i1 = digits1.iter().position_max().unwrap();
    let d1 = digits1[i1];
    let i1 = digits1.iter().position(|&x| x == d1).unwrap();

    let digits2 = &digits[i1 + 1..digits.len()];
    let i2 = digits2.iter().position_max().unwrap();
    let d2 = digits2[i2];

    d1 * 10 + d2
}

fn solve(input_file: &str) -> usize {
    input_file
        .lines()
        .map(|x| {
            find_joltage(
                &x.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn find_joltage_12(digits: &[usize]) -> usize {
    let mut current_index = 0;
    let mut selected_digits = Vec::new();
    for i in 0..12 {
        let digits1 = &digits[current_index..digits.len() + i - 11];
        let i1 = digits1.iter().position_max().unwrap();
        let d1 = digits1[i1];
        let i1 = digits1.iter().position(|&x| x == d1).unwrap();
        current_index += i1 + 1;
        selected_digits.push(d1);
    }

    selected_digits.iter().fold(0, |acc, x| acc * 10 + x)
}

fn solve2(input_file: &str) -> usize {
    input_file
        .lines()
        .map(|x| {
            find_joltage_12(
                &x.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn main() {
    let input_file = fs::read_to_string("input03.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

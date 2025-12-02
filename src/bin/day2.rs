use advent_of_code_2025::parse_utils;
use itertools::Itertools;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(1227775554, r);
    }

    #[test]
    pub fn test2() {
        assert_eq!(false, is_repeated2(101));
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(4174379265, r);
    }
}

fn is_repeated(x: usize) -> bool {
    let x = format!("{}", x);
    if x.len() % 2 != 0 {
        return false;
    }
    let a = &x[0..x.len() / 2];
    let b = &x[x.len() / 2..];
    a == b
}

fn get_invalid_number(x: (usize, usize)) -> Vec<usize> {
    let (a, b) = x;
    let mut invalid_numbers = Vec::new();
    for x in a..=b {
        if is_repeated(x) {
            // println!("Found invalid number: {}", x);
            invalid_numbers.push(x);
        }
    }
    invalid_numbers
}

fn solve(input_file: &str) -> usize {
    let xs = parse_utils::parse_numbers::<usize>(input_file);
    let xs = xs.into_iter().tuples::<(usize, usize)>();
    xs.into_iter()
        .map(|x| get_invalid_number(x).iter().sum::<usize>())
        .sum::<usize>()
}

fn is_repeated2(x: usize) -> bool {
    let x = format!("{}", x);
    let n = x.len();
    for i in 1..=n / 2 {
        if n % i == 0 {
            let patt = &x[0..i];
            let mut start = i;
            let mut all_match = true;
            while start < n {
                let c = &x[start..start + i];
                if c != patt {
                    all_match = false;
                    break;
                }
                start += i;
            }
            if all_match {
                return true;
            }
        }
    }
    false
}

fn get_invalid_number2(x: (usize, usize)) -> Vec<usize> {
    let (a, b) = x;
    let mut invalid_numbers = Vec::new();
    for x in a..=b {
        if is_repeated2(x) {
            // println!("Found invalid number: {}", x);
            invalid_numbers.push(x);
        }
    }
    invalid_numbers
}

fn solve2(input_file: &str) -> usize {
    let xs = parse_utils::parse_numbers::<usize>(input_file);
    let xs = xs.into_iter().tuples::<(usize, usize)>();
    xs.into_iter()
        .map(|x| get_invalid_number2(x).iter().sum::<usize>())
        .sum::<usize>()
}

fn main() {
    let input_file = fs::read_to_string("input02.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

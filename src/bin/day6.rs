use advent_of_code_2025::parse_utils;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(4277556, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(3263827, r);
    }
}

fn solve(input_file: &str) -> isize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let numbers = &lines[0..lines.len() - 1]
        .iter()
        .map(|line| parse_utils::parse_numbers::<isize>(line))
        .collect::<Vec<_>>();
    let ops = lines[lines.len() - 1]
        .replace(" ", "")
        .chars()
        .collect::<Vec<_>>();

    let mut numbers_t = Vec::new();
    for col in 0..numbers[0].len() {
        let mut xs = Vec::new();
        for number_row in numbers {
            xs.push(number_row[col]);
        }
        numbers_t.push(xs);
    }
    let numbers = numbers_t;

    let mut results = Vec::new();
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => {
                results.push(numbers[i].iter().sum::<isize>());
            }
            '*' => {
                results.push(numbers[i].iter().product::<isize>());
            }
            _ => {
                panic!("Invalid operator: {}", op);
            }
        }
    }

    results.iter().sum::<isize>()
}

fn parse_columns(grid: &[Vec<char>]) -> Vec<Vec<isize>> {
    let mut nss = Vec::new();
    let mut ns = Vec::new();
    for col in 0..grid[0].len() {
        let mut ys = Vec::new();
        for grid_row in grid {
            ys.push(grid_row[col]);
        }
        let ys = parse_utils::parse_numbers::<isize>(&ys.iter().collect::<String>());
        if ys.is_empty() {
            nss.push(ns);
            ns = Vec::new();
        } else {
            ns.push(ys[0])
        }
    }
    nss.push(ns);

    nss
}

fn solve2(input_file: &str) -> isize {
    let lines = input_file.lines().collect::<Vec<_>>();
    let grid = lines[0..lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ops = lines[lines.len() - 1]
        .replace(" ", "")
        .chars()
        .collect::<Vec<_>>();
    let numbers = parse_columns(&grid);
    assert_eq!(ops.len(), numbers.len());

    let mut results = Vec::new();
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => {
                results.push(numbers[i].iter().sum::<isize>());
            }
            '*' => {
                results.push(numbers[i].iter().product::<isize>());
            }
            _ => {
                panic!("Invalid operator: {}", op);
            }
        }
    }

    results.iter().sum::<isize>()
}

fn main() {
    let input_file = fs::read_to_string("input06.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

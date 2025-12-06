use std::{cmp::max, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(3, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(14, r);
    }
}

fn is_fresh(id: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|&(a, b)| id >= a && id <= b)
}

fn parse(input_file: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut fresh_ranges = Vec::new();
    let mut available_ingredients = Vec::new();
    let mut parsing_ranges = true;
    for line in input_file.lines() {
        if parsing_ranges {
            if line.is_empty() {
                parsing_ranges = false;
                continue;
            }
            let xs = line.split("-").collect::<Vec<_>>();
            let (a, b) = (
                xs[0].parse::<usize>().unwrap(),
                xs[1].parse::<usize>().unwrap(),
            );
            fresh_ranges.push((a, b));
        } else {
            available_ingredients.push(line.parse::<usize>().unwrap());
        }
    }
    (fresh_ranges, available_ingredients)
}

fn solve(input_file: &str) -> usize {
    let (fresh_ranges, available_ingredients) = parse(input_file);
    available_ingredients
        .into_iter()
        .filter(|&id| is_fresh(id, &fresh_ranges))
        .count()
}

fn solve2(input_file: &str) -> usize {
    let (fresh_ranges, _) = parse(input_file);
    let mut fresh_ranges = fresh_ranges
        .into_iter()
        .map(|(a, b)| (a, b + 1))
        .collect::<Vec<_>>();

    loop {
        fresh_ranges.sort_by_key(|&(a, _)| a);
        let mut did_combine = false;
        for i in 1..fresh_ranges.len() {
            if fresh_ranges[i - 1].1 < fresh_ranges[i].0 {
                continue;
            }
            fresh_ranges[i - 1].1 = max(fresh_ranges[i - 1].1, fresh_ranges[i].1);
            fresh_ranges.remove(i);
            did_combine = true;
            break;
        }

        if !did_combine {
            break;
        }
    }

    fresh_ranges.iter().map(|&(a, b)| b - a).sum()
}

fn main() {
    let input_file = fs::read_to_string("input05.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);
    //
    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

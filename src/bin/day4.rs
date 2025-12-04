use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(13, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(43, r);
    }
}

type Vec2i = (isize, isize);

fn read_at(map: &[Vec<char>], p: Vec2i) -> char {
    let (x, y) = p;
    if x < 0 || y < 0 || x >= map.len() as isize || y >= map[0].len() as isize {
        return '.';
    }
    map[y as usize][x as usize]
}

fn solve(input_file: &str) -> isize {
    let xss = input_file
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for y in 0..xss.len() {
        for x in 0..xss[0].len() {
            let mut adj_count = 0;
            if read_at(&xss, (x as isize, y as isize)) != '@' {
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if read_at(&xss, (x as isize + dx, y as isize + dy)) == '@' {
                        adj_count += 1;
                    }
                }
            }
            if adj_count < 4 {
                count += 1;
            }
        }
    }
    count
}

fn remove_rolls(xss: &[Vec<char>]) -> (usize, Vec<Vec<char>>) {
    let mut output_map = Vec::new();
    let mut count = 0;
    for y in 0..xss.len() {
        let mut output_row = Vec::new();
        for x in 0..xss[0].len() {
            let mut adj_count = 0;
            if read_at(xss, (x as isize, y as isize)) != '@' {
                output_row.push(xss[y][x]);
                continue;
            }
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if read_at(xss, (x as isize + dx, y as isize + dy)) == '@' {
                        adj_count += 1;
                    }
                }
            }
            if adj_count < 4 {
                output_row.push('.');
                count += 1;
            } else {
                output_row.push(xss[y][x]);
            }
        }
        output_map.push(output_row);
    }
    (count, output_map)
}

fn solve2(input_file: &str) -> usize {
    let mut xss = input_file
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total_count = 0;
    loop {
        let (count, output_map) = remove_rolls(&xss);
        if count == 0 {
            break;
        }
        xss = output_map;
        total_count += count;
    }
    total_count
}

fn main() {
    let input_file = fs::read_to_string("input04.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

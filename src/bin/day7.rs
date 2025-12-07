use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(21, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(40, r);
    }
}

type Vec2i = (isize, isize);

fn solve(input_file: &str) -> isize {
    let diagram = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut q: VecDeque<Vec2i> = VecDeque::new();
    q.push_back(((diagram[0].len() / 2) as isize, 0_isize));
    let mut visited = vec![false; diagram.len() * diagram[0].len()];
    let mut split_count = 0;
    while let Some((x, y)) = q.pop_front() {
        if y >= diagram.len() as isize {
            continue;
        }
        if x < 0 || x >= diagram[y as usize].len() as isize {
            continue;
        }
        if visited[y as usize * diagram[0].len() + x as usize] {
            continue;
        }
        visited[y as usize * diagram[0].len() + x as usize] = true;

        if diagram[y as usize][x as usize] == '^' {
            q.push_back((x - 1, y));
            q.push_back((x + 1, y));
            split_count += 1;
        } else {
            q.push_back((x, y + 1));
        }
    }

    split_count
}

fn next_row(
    particles: &HashMap<usize, usize>,
    diagram: &[Vec<char>],
    y: usize,
) -> HashMap<usize, usize> {
    let mut new_particles = HashMap::new();
    for (&x, &n) in particles {
        if diagram[y][x] == '^' {
            new_particles
                .entry(x - 1)
                .and_modify(|m| *m += n)
                .or_insert(n);
            new_particles
                .entry(x + 1)
                .and_modify(|m| *m += n)
                .or_insert(n);
        } else {
            new_particles.entry(x).and_modify(|m| *m += n).or_insert(n);
        }
    }
    new_particles
}

fn solve2(input_file: &str) -> usize {
    let diagram = input_file
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let particles = HashMap::from([(diagram[0].len() / 2, 1)]);
    (1..diagram.len())
        .fold(particles, |acc, y| next_row(&acc, &diagram, y))
        .values()
        .sum()
}

fn main() {
    let input_file = fs::read_to_string("input07.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

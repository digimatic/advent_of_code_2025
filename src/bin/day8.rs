use advent_of_code_2025::parse_utils;
use std::{cmp::Reverse, collections::HashSet, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT, Some(10), None, false);
        assert_eq!(40, r);
    }

    #[test]
    pub fn test2() {
        let r = solve(EXAMPLE1_INPUT, None, None, true);
        assert_eq!(25272, r);
    }
}

type Vec3i = (isize, isize, isize);

fn dist_sqr(a: &Vec3i, b: &Vec3i) -> isize {
    let (x1, y1, z1) = a;
    let (x2, y2, z2) = b;
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)
}

fn solve(input_file: &str, count: Option<usize>, mul_count: Option<usize>, part2: bool) -> usize {
    let xs = input_file
        .lines()
        .map(parse_utils::parse_numbers::<isize>)
        .map(|v| (v[0], v[1], v[2]))
        .collect::<Vec<_>>();
    let mut distances = Vec::new();
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            let d = dist_sqr(&xs[i], &xs[j]);
            distances.push((d, i, j));
        }
    }
    distances.sort_by_key(|(d, _, _)| *d);
    let mut sets: Vec<HashSet<usize>> = Vec::new();
    let distances = if part2 {
        &distances
    } else {
        &distances[0..count.unwrap()]
    };
    for &(_, i, j) in distances {
        let k1 = sets.iter().position(|x| x.contains(&i));
        let k2 = sets.iter().position(|x| x.contains(&j));

        if let Some(k1) = k1 {
            if let Some(k2) = k2 {
                if k1 != k2 {
                    let (first_idx, second_idx) = if k1 > k2 { (k1, k2) } else { (k2, k1) };
                    let mut new_set = sets.remove(first_idx);
                    new_set.extend(sets.remove(second_idx).drain());
                    assert!(!new_set.is_empty());
                    sets.push(new_set);
                }
            } else {
                sets[k1].insert(i);
                sets[k1].insert(j);
            }
        } else if let Some(k2) = k2 {
            sets[k2].insert(i);
            sets[k2].insert(j);
        } else {
            sets.push(HashSet::from([i, j]));
        }

        if part2 && sets.len() == 1 && sets.first().unwrap().len() == xs.len() {
            // Part 2 solution
            return (xs[i].0 * xs[j].0) as usize;
        }
    }

    assert!(!part2);

    if let Some(mul_count) = mul_count {
        let mut len_sets = sets.iter().map(|s| s.len()).collect::<Vec<_>>();
        len_sets.sort_by_key(|&x| Reverse(x));
        len_sets[0..mul_count].iter().product::<usize>()
    } else {
        let len_sets = sets
            .iter()
            .map(|s| s.len())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        len_sets.into_iter().product()
    }
}

fn main() {
    let input_file = fs::read_to_string("input08.txt").unwrap();

    let r = solve(&input_file, Some(1000), Some(3), false);
    println!("Part 1: {}", r); // 840 to low

    let r = solve(&input_file, None, None, true);
    println!("Part 2: {}", r);
}

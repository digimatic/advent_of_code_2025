use std::fs;
use advent_of_code_2025::parse_utils;

//#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

    //#[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(-1, r);
    }

    //#[test]
    // pub fn test2() {
    //     let r = solve2(EXAMPLE2_INPUT);
    //     assert_eq!("", r);
    // }

    //#[test]
    // pub fn test3() {
    //     let r = solve3(EXAMPLE3_INPUT);
    //     assert_eq!("", r);
    // }

    //#[test]
    // pub fn test4() {
    //     let r = solve3(EXAMPLE4_INPUT);
    //     assert_eq!("", r);
    // }
}

#[allow(dead_code)]
type Vec2i = (isize, isize);

fn solve(input_file: &str) -> isize {
    // let lines = input_file.lines().collect::<Vec<_>>();
    let xss = input_file
        .lines()
        .map(|x| parse_utils::parse_signed_numbers::<isize>(x))
        .collect::<Vec<_>>();
    for xs in xss {
        for x in xs {
            println!("{}", x);
        }
    }

    0
}

fn main() {
    tests::test1();
    // tests::test2();
    // tests::test3();
    // tests::test4();

    let input_file = fs::read_to_string("input01.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    // let r = solve2(&input_file);
    // println!("Part 2: {}", r);

    // let r = solve3(&input_file);
    // println!("Part 3: {}", r);
}

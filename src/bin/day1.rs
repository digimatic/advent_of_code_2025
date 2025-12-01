use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    pub fn test_do_turn_right() {
        assert_eq!((50, 10), do_turn(50, 'R', 1000));
        assert_eq!((51, 0), do_turn(50, 'R', 1));
        assert_eq!((1, 1), do_turn(99, 'R', 2));
        assert_eq!((0, 1), do_turn(99, 'R', 1));
        assert_eq!((10, 0), do_turn(0, 'R', 10));
        assert_eq!((10, 1), do_turn(0, 'R', 110));
    }

    #[test]
    pub fn test_do_turn_left() {
        assert_eq!((50, 1), do_turn(50, 'L', 100));
        assert_eq!((50, 2), do_turn(50, 'L', 200));
        assert_eq!((99, 1), do_turn(1, 'L', 2));
        assert_eq!((48, 0), do_turn(50, 'L', 2));
        assert_eq!((98, 0), do_turn(0, 'L', 2));
        assert_eq!((0, 1), do_turn(0, 'L', 100));
    }

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(3, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(6, r);
    }
}

fn solve(input_file: &str) -> isize {
    let xs = parse(input_file);

    let mut current = 50;
    let mut zero_count = 0;
    for (dir, x) in xs {
        current += match dir {
            'L' => -x,
            'R' => x,
            _ => panic!("Unexpected dir: {}", dir),
        };
        current = ((current % 100) + 100) % 100;
        if current == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn parse(input_file: &str) -> Vec<(char, isize)> {
    input_file
        .lines()
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<isize>().unwrap()))
        .collect::<Vec<_>>()
}

fn do_turn(mut current: isize, dir: char, x: isize) -> (isize, isize) {
    let prev = current;
    // print!("From {} : {}-{} -> ", prev, dir, x);
    let x100 = x / 100;
    let xrest = x % 100;
    let mut n = 0;
    if xrest != 0 {
        current += match dir {
            'L' => -xrest,
            'R' => xrest,
            _ => panic!("Unexpected dir: {}", dir),
        };
        if current == 0 {
            n += 1;
        }
        while current < 0 {
            current += 100;
            n += 1;
        }
        if n > 0 && prev == 0 {
            n -= 1;
        }
        while current >= 100 {
            current -= 100;
            n += 1;
        }
    }
    let n = n + x100;

    // println!("-> rotated to point at {}, 0 #{} times", current, n);
    (current, n)
}

fn solve2(input_file: &str) -> isize {
    let xs = parse(input_file);

    let mut current = 50;
    let mut zero_count = 0;
    for (dir, x) in xs {
        let (new_current, n) = do_turn(current, dir, x);
        current = new_current;
        zero_count += n;
    }

    zero_count
}

fn main() {
    let input_file = fs::read_to_string("input01.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);
    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

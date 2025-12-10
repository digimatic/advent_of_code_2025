use advent_of_code_2025::parse_utils::parse_numbers;
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
};
use itertools::Itertools;
use regex::Regex;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    #[allow(dead_code)]
    const EXAMPLE2_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE3_INPUT: &str = r"";
    #[allow(dead_code)]
    const EXAMPLE4_INPUT: &str = r"";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(7, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE1_INPUT);
        assert_eq!(33, r);
    }
}

struct Machine {
    dimension: usize,
    lights_mask: usize,
    wiring_masks: Vec<usize>,
    joltage: Vec<usize>,
}

fn parse_machine(input: &str) -> Machine {
    let re = Regex::new(r"\[([.#]+)\]((?: \(\d+(?:,\d+)*\))+) \{(\d+(?:,\d+)*)\}").unwrap();
    let caps = re.captures(input).unwrap();
    let lights: Vec<bool> = caps[1].to_string().chars().map(|x| x == '#').collect();
    let dimension = lights.len();
    let wiring: Vec<Vec<usize>> = caps[2]
        .trim()
        .split(" ")
        .map(parse_numbers::<usize>)
        .collect();
    let joltage = parse_numbers::<usize>(caps[3].to_string().as_str());
    let lights_mask = bool_vec_to_mask(&lights);
    let wiring_masks = wiring.iter().map(|x| indexes_to_mask(x)).collect();
    Machine {
        dimension,
        lights_mask,
        wiring_masks,
        joltage,
    }
}

fn parse_machines(input_file: &str) -> Vec<Machine> {
    input_file.lines().map(parse_machine).collect::<Vec<_>>()
}

fn bool_vec_to_mask(xs: &[bool]) -> usize {
    let mut mask = 0;
    for x in xs.iter().rev() {
        mask <<= 1;
        if *x {
            mask |= 1;
        }
    }
    mask
}

fn indexes_to_mask(xs: &[usize]) -> usize {
    let mut mask = 0;
    for x in xs {
        mask |= 1 << x;
    }
    mask
}

fn find_fewest_presses(machine: &Machine) -> usize {
    if machine.lights_mask == 0 {
        return 0;
    }
    let all_button_indices = (0..machine.wiring_masks.len()).collect_vec();
    for n in 1..=machine.dimension {
        for indices in all_button_indices.iter().combinations(n) {
            let indices_vec: Vec<usize> = indices.into_iter().copied().collect();
            let res = indices_vec
                .iter()
                .fold(0, |c, &i| c ^ machine.wiring_masks[i]);
            if res == machine.lights_mask {
                return n;
            }
        }
    }

    panic!("No solution found");
}

fn ilp(dim: usize, buttons: &[usize], joltages: &[usize]) -> Vec<usize> {
    let a: Vec<Vec<f64>> = (0..dim)
        .map(|j| {
            buttons
                .iter()
                .map(|&button| if (button >> j) & 1 == 1 { 1.0 } else { 0.0 })
                .collect()
        })
        .collect();
    let b: Vec<f64> = joltages.iter().map(|&x| x as f64).collect();

    let mut problem = ProblemVariables::new();
    let x: Vec<_> = (0..buttons.len())
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = x.iter().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    for (row, &rhs) in a.iter().zip(&b) {
        let lhs: Expression = row.iter().zip(&x).map(|(&coef, &var)| coef * var).sum();
        model = model.with(constraint!(lhs == rhs));
    }

    let solution = model.solve().expect("No solution");

    x.iter()
        .map(|&var| (solution.value(var) + 0.5) as usize)
        .collect()
}

fn solve(input_file: &str) -> usize {
    let machines = parse_machines(input_file);
    machines.iter().map(find_fewest_presses).sum()
}

fn solve2(input_file: &str) -> usize {
    let machines = parse_machines(input_file);
    machines
        .iter()
        .map(|m| {
            ilp(m.dimension, &m.wiring_masks, &m.joltage)
                .iter()
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input_file = fs::read_to_string("input10.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

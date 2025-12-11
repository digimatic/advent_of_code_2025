use std::{collections::HashMap, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    const EXAMPLE2_INPUT: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(5, r);
    }

    #[test]
    pub fn test2() {
        let r = solve2(EXAMPLE2_INPUT);
        assert_eq!(2, r);
    }
}

fn traverse_all(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> usize {
    if start == end {
        return 1;
    }

    graph[start]
        .iter()
        .map(|c| traverse_all(graph, c, end))
        .sum()
}

fn solve(input_file: &str) -> usize {
    let graph = parse(input_file);
    traverse_all(&graph, "you", "out")
}

fn parse(input_file: &str) -> HashMap<&str, Vec<&str>> {
    input_file
        .lines()
        .map(|line| {
            let xs = line.split(": ").collect::<Vec<_>>();
            let ys = xs[1].split(" ").collect::<Vec<_>>();
            (xs[0], ys)
        })
        .collect::<HashMap<_, _>>()
}

fn traverse_all_2(
    graph: &HashMap<&str, Vec<&str>>,
    start: &str,
    end: &str,
    dac_visited: bool,
    fft_visited: bool,
    cache: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if let Some(v) = cache.get(&(start.to_string(), dac_visited, fft_visited)) {
        return *v;
    }

    if start == end {
        if dac_visited && fft_visited {
            return 1;
        } else {
            return 0;
        }
    }
    let dac_visited = dac_visited || start == "dac";
    let fft_visited = fft_visited || start == "fft";
    let s = graph[start]
        .iter()
        .map(|c| traverse_all_2(graph, c, end, dac_visited, fft_visited, cache))
        .sum();
    cache.insert((start.to_string(), dac_visited, fft_visited), s);
    s
}

fn solve2(input_file: &str) -> usize {
    let graph = parse(input_file);
    traverse_all_2(&graph, "svr", "out", false, false, &mut HashMap::new())
}
fn main() {
    let input_file = fs::read_to_string("input11.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

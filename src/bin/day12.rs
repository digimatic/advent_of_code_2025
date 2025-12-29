use advent_of_code_2025::parse_utils::parse_numbers;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(2, r);
    }
}

type Vec2i = (isize, isize);
type Shape = Vec<Vec<char>>;
type Region = (Vec2i, Vec<usize>);
type Canvas = Vec<Vec<char>>;

fn parse(input_file: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines = input_file.lines().collect::<Vec<_>>();
    let mut line_index = 0;
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();
    loop {
        if line_index >= lines.len() {
            break;
        }
        let line = lines[line_index];
        if line.ends_with(":") {
            let shape_row1 = lines[line_index + 1].chars().collect::<Vec<char>>();
            let shape_row2 = lines[line_index + 2].chars().collect::<Vec<char>>();
            let shape_row3 = lines[line_index + 3].chars().collect::<Vec<char>>();
            let shape = vec![shape_row1, shape_row2, shape_row3];
            shapes.push(shape);
            line_index += 5;
        } else {
            let s1 = line.split(": ").collect::<Vec<_>>();
            let sizes = parse_numbers::<isize>(s1[0]);
            let counts = parse_numbers::<usize>(s1[1]);
            line_index += 1;

            regions.push(((sizes[0], sizes[1]), counts));
        }
    }
    (shapes, regions)
}

const TRANSFORMS: [[[isize; 2]; 2]; 8] = [
    [[1, 0], [0, 1]],   // identity
    [[0, 1], [-1, 0]],  // 90° rotation
    [[-1, 0], [0, -1]], // 180° rotation
    [[0, -1], [1, 0]],  // 270° rotation
    [[-1, 0], [0, 1]],  // horizontal flip
    [[1, 0], [0, -1]],  // vertical flip
    [[0, 1], [1, 0]],   // diagonal flip
    [[0, -1], [-1, 0]], // anti-diagonal flip
];

fn transform_shape(shape: &Shape, transform: &[[isize; 2]; 2]) -> Shape {
    let mut result = vec![vec!['.'; 3]; 3];

    for (row, shape_row) in shape.iter().enumerate() {
        for (col, &cell) in shape_row.iter().enumerate() {
            let centered_row = row as isize - 1;
            let centered_col = col as isize - 1;

            let new_row = transform[0][0] * centered_row + transform[0][1] * centered_col;
            let new_col = transform[1][0] * centered_row + transform[1][1] * centered_col;

            let adjusted_row = new_row + 1;
            let adjusted_col = new_col + 1;

            if (0..3).contains(&adjusted_row) && (0..3).contains(&adjusted_col) {
                result[adjusted_row as usize][adjusted_col as usize] = cell;
            }
        }
    }

    result
}

fn all_variants(shape: &Shape) -> Vec<Shape> {
    let mut variants = Vec::new();

    for transform in &TRANSFORMS {
        let variant = transform_shape(shape, transform);
        variants.push(variant);
    }

    variants.sort();
    variants.dedup();

    variants
}

fn all_shape_variants(shapes: &[Shape]) -> Vec<Vec<Shape>> {
    shapes.iter().map(all_variants).collect()
}

fn try_draw_shape_at(shape: &Shape, pos: Vec2i, canvas: &Canvas, symbol: char) -> Option<Canvas> {
    let (start_x, start_y) = pos;

    for (row_idx, row) in shape.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell != '.' {
                let y = start_y + row_idx as isize;
                let x = start_x + col_idx as isize;

                if y < 0 || x < 0 || y >= canvas.len() as isize || x >= canvas[0].len() as isize {
                    return None;
                }

                if canvas[y as usize][x as usize] != '.' {
                    return None;
                }
            }
        }
    }

    let mut new_canvas = canvas.clone();

    for (row_idx, row) in shape.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell != '.' {
                let y = start_y + row_idx as isize;
                let x = start_x + col_idx as isize;
                assert!(y >= 0 && y < new_canvas.len() as isize);
                assert!(x >= 0 && x < new_canvas[0].len() as isize);
                assert_eq!('.', new_canvas[y as usize][x as usize]);
                new_canvas[y as usize][x as usize] = symbol;
            }
        }
    }

    Some(new_canvas)
}

fn next_region(region: &Region) -> Option<(Region, usize)> {
    let (size, counts) = region;
    let mut new_counts = counts.clone();

    for (i, &count) in counts.iter().enumerate() {
        if count > 0 {
            new_counts[i] = count - 1;
            return Some(((*size, new_counts), i));
        }
    }

    None
}

fn shape_area(shape: &Shape) -> usize {
    shape
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn solve_region(
    shapes: &[Vec<Shape>],
    region: &Region,
    canvas: &Canvas,
    symbol: char,
) -> Option<Canvas> {
    if let Some((next_region, next_idx)) = next_region(region) {
        let shape_variants = &shapes[next_idx];
        for variant in shape_variants.iter() {
            let (width, height) = region.0;
            for y in 0..(height - 2) {
                for x in 0..(width - 2) {
                    if let Some(new_canvas) = try_draw_shape_at(variant, (x, y), canvas, symbol) {
                        let next_symbol = if symbol == 'Z' {
                            '#'
                        } else {
                            (symbol as u8 + 1) as char
                        };

                        if let Some(result_canvas) =
                            solve_region(shapes, &next_region, &new_canvas, next_symbol)
                        {
                            return Some(result_canvas);
                        }
                    }
                }
            }
        }
        None
    } else {
        Some(canvas.clone())
    }
}

#[allow(dead_code)]
fn print_canvas(canvas: &Canvas) {
    for row in canvas {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

#[allow(dead_code)]
fn print_shape_variants(shape_variants: &[Vec<Shape>]) {
    for (shape_idx, variants) in shape_variants.iter().enumerate() {
        println!("Shape {} variants:", shape_idx);
        for (variant_idx, variant) in variants.iter().enumerate() {
            println!("  Variant {}:", variant_idx);
            for row in variant {
                println!("    {}", row.iter().collect::<String>());
            }
            println!();
        }
        println!();
    }
}

fn solve(input_file: &str) -> isize {
    let (shapes, regions) = parse(input_file);
    // println!("Shapes: {:?}", shapes);
    // println!("Regions: {:?}", regions);
    let shape_variants = all_shape_variants(&shapes);

    // print_shape_variants(&shape_variants);

    regions
        .iter()
        .map(|region| {
            let empty_canvas = vec![vec!['.'; region.0.0 as usize]; region.0.1 as usize];

            let area_required = region
                .1
                .iter()
                .enumerate()
                .map(|(i, &count)| shape_area(&shape_variants[i][0]) * count)
                .sum::<usize>();

            let canvas_area = (region.0.0 * region.0.1) as usize;
            if canvas_area < area_required {
                return None;
            }

            // The fitting part below is not needed for actual input. Only for the example input.

            let canvas_3x3_blocks = ((region.0.0 / 3) * (region.0.1 / 3)) as usize;
            let num_3x3_blocks: usize = region.1.iter().sum();
            if num_3x3_blocks <= canvas_3x3_blocks {
                return Some(vec![vec!['?'; region.0.0 as usize]; region.0.1 as usize]);
            }

            solve_region(&shape_variants, region, &empty_canvas, 'A')
        })
        .filter(|result| result.is_some())
        .count() as isize
}

fn main() {
    let input_file = fs::read_to_string("input12.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);
}

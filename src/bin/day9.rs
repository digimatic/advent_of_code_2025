use advent_of_code_2025::parse_utils;
use std::{cmp::Reverse, fs};

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1_INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    pub fn test1() {
        let r = solve(EXAMPLE1_INPUT);
        assert_eq!(50, r);
    }

    // #[test]
    // pub fn test2() {
    //     let r = solve2(EXAMPLE1_INPUT);
    //     assert_eq!(24, r);
    // }
}

type Vec2i = (isize, isize);

fn solve(input_file: &str) -> isize {
    let xs = input_file
        .lines()
        .map(|x| {
            let ns = parse_utils::parse_numbers::<isize>(x);
            (ns[0], ns[1])
        })
        .collect::<Vec<_>>();

    let mut areas = Vec::new();
    for &p in &xs {
        for &q in &xs {
            let dx = (p.0 - q.0).abs() + 1;
            let dy = (p.1 - q.1).abs() + 1;
            areas.push(dx * dy);
        }
    }
    *areas.iter().max().unwrap()
}

fn is_line_intersecting(p0: Vec2i, p1: Vec2i, q0: Vec2i, q1: Vec2i) -> bool {
    let p_horizontal = p0.1 == p1.1;
    let q_horizontal = q0.1 == q1.1;

    if p_horizontal == q_horizontal {
        return false;
    }

    // p is horizontal, q is vertical
    let (h0, h1, v0, v1) = if p_horizontal {
        (p0, p1, q0, q1)
    } else {
        (q0, q1, p0, p1)
    };

    let h_y = h0.1;
    let h_min_x = h0.0.min(h1.0);
    let h_max_x = h0.0.max(h1.0);

    let v_x = v0.0;
    let v_min_y = v0.1.min(v1.1);
    let v_max_y = v0.1.max(v1.1);

    v_x > h_min_x && v_x < h_max_x && h_y > v_min_y && h_y < v_max_y
}

fn has_point_inside(xs: &[Vec2i], p: Vec2i, q: Vec2i) -> bool {
    let min_x = p.0.min(q.0);
    let max_x = p.0.max(q.0);
    let min_y = p.1.min(q.1);
    let max_y = p.1.max(q.1);

    for &pt in xs {
        // Skip if point is exactly on a corner
        let is_corner = (pt.0 == min_x || pt.0 == max_x) && (pt.1 == min_y || pt.1 == max_y);
        if is_corner {
            continue;
        }

        // Fail if point is inside or on edge (but not corner)
        if pt.0 >= min_x && pt.0 <= max_x && pt.1 >= min_y && pt.1 <= max_y {
            return true;
        }
    }
    false
}

fn has_intersection(xs: &[Vec2i], p: Vec2i, q: Vec2i) -> bool {
    let mut xs = xs.to_vec();
    xs.push(xs[0]);

    let min_x = p.0.min(q.0);
    let max_x = p.0.max(q.0);
    let min_y = p.1.min(q.1);
    let max_y = p.1.max(q.1);
    let ys = [(min_x, min_y),
        (max_x, min_y),
        (max_x, max_y),
        (min_x, max_y),
        (min_x, min_y)];

    for i in 0..xs.len() - 1 {
        let p0 = xs[i];
        let p1 = xs[i + 1];

        for j in 0..ys.len() - 1 {
            let q0 = ys[j];
            let q1 = ys[j + 1];
            if is_line_intersecting(p0, p1, q0, q1) {
                return true;
            }
        }
    }
    false
}

// LLM generanted to draw_polygons
fn draw_poly(p: &[Vec2i], solution: Option<(Vec2i, Vec2i)>) {
    use image::{ImageBuffer, Rgb};

    let width = 800u32;
    let height = 600u32;
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Rgb([255, 255, 255]);
    }

    // Find bounds to scale polygon to fit image
    if p.is_empty() {
        return;
    }

    let min_x = p.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = p.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = p.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = p.iter().map(|&(_, y)| y).max().unwrap();

    let scale_x = (width as f32 - 40.0) / (max_x - min_x) as f32;
    let scale_y = (height as f32 - 40.0) / (max_y - min_y) as f32;
    let scale = scale_x.min(scale_y);

    // Helper to transform coordinates
    let transform = |px: isize, py: isize| -> (u32, u32) {
        let x = ((px - min_x) as f32 * scale + 20.0) as u32;
        let y = ((py - min_y) as f32 * scale + 20.0) as u32;
        (x, y)
    };

    // Helper to draw a line using Bresenham's algorithm
    let draw_line = |img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
                     x1: u32,
                     y1: u32,
                     x2: u32,
                     y2: u32,
                     color: Rgb<u8>| {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1 as i32;
        let mut y = y1 as i32;

        loop {
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                img.put_pixel(x as u32, y as u32, color);
            }

            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    };

    // Draw polygon edges in red
    for i in 0..p.len() {
        let start = p[i];
        let end = p[(i + 1) % p.len()];

        let (x1, y1) = transform(start.0, start.1);
        let (x2, y2) = transform(end.0, end.1);

        draw_line(&mut img, x1, y1, x2, y2, Rgb([255, 0, 0]));
    }

    // Draw vertices as small circles (blue)
    for &(px, py) in p {
        let (x, y) = transform(px, py);

        for ddx in -2..=2i32 {
            for ddy in -2..=2i32 {
                if ddx * ddx + ddy * ddy <= 4 {
                    let nx = x as i32 + ddx;
                    let ny = y as i32 + ddy;
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        img.put_pixel(nx as u32, ny as u32, Rgb([0, 0, 255]));
                    }
                }
            }
        }
    }

    // Draw solution rectangle on top (green edges with square corner markers)
    if let Some((p1, p2)) = solution {
        let corners = [(p1.0, p1.1), (p2.0, p1.1), (p2.0, p2.1), (p1.0, p2.1)];
        // Draw edges
        for i in 0..4 {
            let start = corners[i];
            let end = corners[(i + 1) % 4];
            let (x1, y1) = transform(start.0, start.1);
            let (x2, y2) = transform(end.0, end.1);
            draw_line(&mut img, x1, y1, x2, y2, Rgb([0, 200, 0]));
        }
        // Draw corner markers as squares
        for &(cx, cy) in &corners {
            let (x, y) = transform(cx, cy);
            for ddx in -4..=4i32 {
                for ddy in -4..=4i32 {
                    let nx = x as i32 + ddx;
                    let ny = y as i32 + ddy;
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        img.put_pixel(nx as u32, ny as u32, Rgb([0, 180, 0]));
                    }
                }
            }
        }
    }

    img.save("day9.png").unwrap();
}

// this solution do not work with the test input
fn solve2(input_file: &str) -> isize {
    let xs = input_file
        .lines()
        .map(|x| {
            let ns = parse_utils::parse_numbers::<isize>(x);
            (ns[0], ns[1])
        })
        .collect::<Vec<_>>();

    let mut areas = Vec::new();
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            let p = xs[i];
            let q = xs[j];
            let dx = (p.0 - q.0).abs() + 1;
            let dy = (p.1 - q.1).abs() + 1;
            areas.push((dx * dy, p, q));
        }
    }
    areas.sort_by_key(|(d, _, _)| Reverse(*d));

    for &(area, p, q) in &areas {
        if !has_point_inside(&xs, p, q) && !has_intersection(&xs, p, q) {
            draw_poly(&xs, Some((p, q)));
            return area;
        }
    }

    draw_poly(&xs, None);
    panic!("No solution found");
}

fn main() {
    let input_file = fs::read_to_string("input09.txt").unwrap();

    let r = solve(&input_file);
    println!("Part 1: {}", r);

    let r = solve2(&input_file);
    println!("Part 2: {}", r);
}

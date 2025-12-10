use crate::Solve;
// use std::{collections::HashMap, result};

pub struct Problem {
    points: Vec<(i64, i64)>,
    areas: Vec<(usize, usize, i64)>,
}
impl Solve for Problem {
    /// 2D points: find the largest area rectangle defined by any two points
    fn p1(&mut self) -> i64 {
        self.areas[0].2
    }

    /// The points form a border
    /// Find the largest rectangle of any 2 points that fits entirely within the border
    fn p2(&mut self) -> i64 {
        // Data is a 100,000 x 100,000 grid
        // There are only about 250 unique x and 250 unique y
        // We need to leave a space to understand fills, so we have about a 500 x 500 grid when compressed
        // To avoid extra Vecs, we'll use a flat Vec and calculate indices manually

        // Get unique x and ys
        let (x_set, y_set) = Self::unique_parts(&self.points);

        // Create the indexes, reducing gaps to only one spot by putting in a filler number, e.g. 1,2,9 is 1,2,3,9
        let x_index = Self::collapse_range(&x_set);
        let y_index = Self::collapse_range(&y_set);

        // Add a buffer to avoid overflows
        let x_len = x_index.len() + 1;
        let y_len = y_index.len() + 1;

        // Create a grid and set the points
        let mut grid: Vec<bool> = vec![false; x_len * y_len];
        for (x, y) in &self.points {
            let xi = x_index.iter().position(|v| v == x).unwrap();
            let yi = y_index.iter().position(|v| v == y).unwrap();
            grid[yi * x_len + xi] = true;
        }
        let mut grid_clone = grid.clone();

        // Fill in the points between the borders
        for yi in 0..y_index.len() {
            let mut in_fill = false;
            let mut last = false;
            for xi in 0..x_len {
                let idx = yi * x_len + xi;

                if grid[idx] && !last {
                    in_fill = !in_fill;
                } else if in_fill {
                    grid_clone[idx] = true;
                }
                last = grid[idx];
            }
        }
        for xi in 0..x_len {
            let mut in_fill = false;
            let mut last = false;
            for yi in 0..y_index.len() {
                let idx = yi * x_len + xi;
                if grid[idx] && !last {
                    in_fill = !in_fill;
                } else if in_fill {
                    grid_clone[idx] = true;
                }
                last = grid[idx];
            }
        }
        grid.clone_from(&grid_clone);
        // good outline here

        // Fill in outlined area in grid_clone
        for yi in 0..y_len {
            let mut in_fill = false;
            let mut last = false;
            for xi in 0..x_len {
                let idx = yi * x_len + xi;
                let left = (yi - 1) * x_len + xi;

                if grid[idx] {
                    if !last {
                        in_fill = !in_fill;
                    }
                } else if in_fill {
                    if grid_clone[left] {
                        grid_clone[idx] = true;
                    } else {
                        in_fill = false;
                    }
                }
                last = grid[idx];
            }
        }

        // Now, check each area to see if it fits within the filled grid
        for (p1, p2, area) in &self.areas {
            let x_min = self.points[*p1].0.min(self.points[*p2].0);
            let x_max = self.points[*p1].0.max(self.points[*p2].0);
            let y_min = self.points[*p1].1.min(self.points[*p2].1);
            let y_max = self.points[*p1].1.max(self.points[*p2].1);

            let xi_min = x_index.iter().position(|v| v == &x_min).unwrap();
            let xi_max = x_index.iter().position(|v| v == &x_max).unwrap();
            let yi_min = y_index.iter().position(|v| v == &y_min).unwrap();
            let yi_max = y_index.iter().position(|v| v == &y_max).unwrap();

            let mut ok = true;
            'outer: for xi in xi_min..=xi_max {
                for yi in yi_min..=yi_max {
                    let idx = yi * x_len + xi;
                    if !grid_clone[idx] {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                return *area;
            }
        }
        0
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        // Calculate combinations  n! / (k! * (n - k)!)
        // Can be simplifed for k=2 items to n * (n - 1) / 2
        let data_len = data.len();
        let combo_len = data_len * (data_len - 1) / 2;

        let mut points: Vec<(i64, i64)> = Vec::with_capacity(data.len());
        let mut areas: Vec<(usize, usize, i64)> = Vec::with_capacity(combo_len);

        // Parse points
        for line in data {
            let mut parts = line.split(',');
            points.push((
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ));
        }

        // Calculate areas for all point combinations
        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                let w = (points[i].0 - points[j].0).abs() + 1;
                let h = (points[i].1 - points[j].1).abs() + 1;
                let a = w * h;
                areas.push((i, j, a));
            }
        }

        // Sort areas descending by area
        areas.sort_by(|a, b| b.2.cmp(&a.2));

        Problem { points, areas }
    }

    fn unique_parts(source: &Vec<(i64, i64)>) -> (Vec<i64>, Vec<i64>) {
        let mut x_points: Vec<i64> = Vec::with_capacity(source.len());
        let mut y_points: Vec<i64> = Vec::with_capacity(source.len());
        for (x, y) in source {
            x_points.push(*x);
            y_points.push(*y);
        }
        x_points.sort_unstable();
        x_points.dedup();
        y_points.sort_unstable();
        y_points.dedup();

        (x_points, y_points)
    }

    fn collapse_range(source: &Vec<i64>) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::with_capacity(source.len() * 2);
        let mut last = -1;
        for v in source {
            if last + 1 < *v {
                result.push(last + 1);
            }
            result.push(*v);
            last = *v;
        }
        result
    }

    fn print_grid(grid: &[bool], x_len: usize, y_len: usize) {
        for yi in 0..y_len {
            for xi in 0..x_len {
                let idx = yi * x_len + xi;
                print!("{}", if grid[idx] { '#' } else { '.' });
            }
            println!();
        }
        println!();
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [50, 24, 4_759_420_470,1_603_439_684];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/09_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/09_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/09.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/09.txt")).p2(), ANSWERS[3]); }
}

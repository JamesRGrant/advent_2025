use crate::Solve;

pub struct Problem {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
}
impl Solve for Problem {
    /// Find locations with less than 4 neighbors
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        Problem::find_lonely(&self.grid, self.h, self.w).len() as i64
    }

    /// Find locations with less than 4 neighbors and remove them
    /// Repeat until stable
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let mut grid = self.grid.clone();
        let mut sum: i64 = 0;

        loop {
            let points = Problem::find_lonely(&grid, self.h, self.w);

            if points.is_empty() {
                break;
            }
            sum += points.len() as i64;

            // Remove those points and look for more
            for (i, j) in points {
                grid[j][i] = '.';
            }
        }

        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let h = data.len();
        let w = data[0].len();

        // To reduce overflow condtions, we are going to make the grid bigger.
        // So, a 4x4 grid becomes 6x6 with a border of '.'
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(h + 2);
        grid.push(vec!['.'; w + 2]);
        for line in data {
            grid.push(format!(".{line}.").chars().collect());
        }
        grid.push(vec!['.'; w + 2]);
        Problem { grid, h, w }
    }

    #[rustfmt::skip]
    fn find_lonely(grid: &[Vec<char>], h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for j in 1..=h {
            for i in 1..=w {
                if grid[j][i] == '@' {
                    let mut count = 0;

                    // Check all 8 neighbors
                    if grid[j - 1][i - 1] == '@' { count += 1; }
                    if grid[j - 1][i    ] == '@' { count += 1; }
                    if grid[j - 1][i + 1] == '@' { count += 1; }
                    if grid[j    ][i - 1] == '@' { count += 1; }
                    if grid[j    ][i + 1] == '@' { count += 1; }
                    if grid[j + 1][i - 1] == '@' { count += 1; }
                    if grid[j + 1][i    ] == '@' { count += 1; }
                    if grid[j + 1][i + 1] == '@' { count += 1; }
                     
                    if count < 4 {
                        points.push((i, j));
                    }
                }
            }
        }
        points
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [13, 43, 1363, 8184];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/04_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/04_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/04.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/04.txt")).p2(), ANSWERS[3]); }
}

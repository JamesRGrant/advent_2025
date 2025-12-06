use crate::Solve;

pub struct Problem {
    grid: Vec<Vec<char>>,
    initial_lonely: Vec<(usize, usize)>,
}
impl Solve for Problem {
    /// Find locations with less than 4 neighbors
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        self.initial_lonely.len() as i64
    }

    /// Find locations with less than 4 neighbors and remove them
    /// Repeat until stable
    /// At the start, there are 18k points.
    /// On each subsequent pass, we will only look at neighbors of those removed
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;

        let mut lonely = self.initial_lonely.clone();
        while !lonely.is_empty() {
            sum += lonely.len() as i64;
            for (i, j) in &lonely {
                self.grid[*j][*i] = '.';
            }

            let mut new_lonely: Vec<(usize, usize)> = Vec::with_capacity(lonely.len() * 2);
            for (i, j) in &lonely {
                Problem::check_neighbors(&self.grid, *i, *j, &mut new_lonely);
            }
            new_lonely.sort_unstable();
            new_lonely.dedup();

            lonely = new_lonely;
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

        // Both problems need this starting list, so do it once
        let initial_lonely = Problem::find_lonely(&grid, h, w);

        Problem { grid, initial_lonely }
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

    #[rustfmt::skip]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    fn check_neighbors(grid: &[Vec<char>], i: usize, j: usize, new_lonely: &mut Vec<(usize, usize)>) {
        for &(di, dj) in &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            let ni = (i as isize + di) as usize;
            let nj = (j as isize + dj) as usize;
            if grid[nj][ni] == '@' {
                let mut count = 0;

                // Check all 8 neighbors
                if grid[nj - 1][ni - 1] == '@' { count += 1; }
                if grid[nj - 1][ni    ] == '@' { count += 1; }
                if grid[nj - 1][ni + 1] == '@' { count += 1; }
                if grid[nj    ][ni - 1] == '@' { count += 1; }
                if grid[nj    ][ni + 1] == '@' { count += 1; }
                if grid[nj + 1][ni - 1] == '@' { count += 1; }
                if grid[nj + 1][ni    ] == '@' { count += 1; }
                if grid[nj + 1][ni + 1] == '@' { count += 1; }

                if count < 4 {
                    new_lonely.push((ni, nj));
                }
            }
        }
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

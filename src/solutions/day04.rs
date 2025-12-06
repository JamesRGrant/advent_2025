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
        let mut sum = 0;
        for j in 1..=self.h {
            for i in 1..=self.w {
                if self.grid[j][i] == '@' && Problem::is_lonely(&self.grid, i, j) {
                    sum += 1;
                }
            }
        }
        sum
    }

    /// Find locations with less than 4 neighbors and remove them
    /// Repeat until stable
    /// At the start, there are 18k points.
    /// On each subsequent pass, we will only look at neighbors of those removed
    #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        let mut grid = self.grid.clone();
        let mut sum: i64 = 0;

        let mut lonely = Problem::find_lonely(&grid, self.h, self.w);
        while !lonely.is_empty() {
            sum += lonely.len() as i64;
            for (i, j) in &lonely {
                grid[*j][*i] = '.';
            }

            let mut new_lonely: Vec<(usize, usize)> = Vec::new();
            for (i, j) in &lonely {
                Problem::check_neighbors(&mut grid, *i, *j, &mut new_lonely);
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
        Problem { grid, h, w }
    }

    #[rustfmt::skip]
    fn find_lonely(grid: &[Vec<char>], h: usize, w: usize) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for j in 1..=h {
            for i in 1..=w {
                if grid[j][i] == '@' && Problem::is_lonely(grid, i, j) {
                        points.push((i, j));
                }
            }
        }
        points
    }

    #[rustfmt::skip]
    fn is_lonely(grid: &[Vec<char>], i: usize, j: usize) -> bool {
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
        
        count < 4
    }


    #[rustfmt::skip]
    fn check_neighbors(grid: &mut [Vec<char>], i: usize, j: usize, new_lonely: &mut Vec<(usize, usize)>) {
        Problem::check_one_neighbor(grid, i - 1, j - 1, new_lonely);
        Problem::check_one_neighbor(grid, i    , j - 1, new_lonely);
        Problem::check_one_neighbor(grid, i + 1, j - 1, new_lonely);
        Problem::check_one_neighbor(grid, i - 1, j    , new_lonely);
        Problem::check_one_neighbor(grid, i + 1, j    , new_lonely);
        Problem::check_one_neighbor(grid, i - 1, j + 1, new_lonely);
        Problem::check_one_neighbor(grid, i    , j + 1, new_lonely);
        Problem::check_one_neighbor(grid, i + 1, j + 1, new_lonely);
    }

    fn check_one_neighbor(grid: &mut [Vec<char>], i: usize, j: usize, new_lonely: &mut Vec<(usize, usize)>) {
        if grid[j][i] == '@' && Problem::is_lonely(grid, i, j) {
            new_lonely.push((i, j));
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

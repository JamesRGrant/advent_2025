use crate::Solve;
use std::collections::HashMap;
pub struct Problem {
    data: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        // Find S
        let mut splits = 0;
        let mut rays: Vec<usize> = vec![self.data[0].iter().position(|&c| c == 'S').unwrap()];

        for row in 1..self.data.len() {
            let mut new_rays: Vec<usize> = Vec::new();

            for &i in &rays {
                if self.data[row][i] == '.' {
                    new_rays.push(i);
                } else {
                    splits += 1;
                    if i > 0 {
                        new_rays.push(i - 1);
                    }
                    if i < self.data[0].len() - 1 {
                        new_rays.push(i + 1);
                    }
                }
            }

            new_rays.sort_unstable();
            new_rays.dedup();
            rays = new_rays;
        }

        splits
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        // Find S
        let mut splits = 0;
        let mut rays: HashMap<usize, usize> = HashMap::new();
        rays.insert(self.data[0].iter().position(|&c| c == 'S').unwrap(), 1);

        for row in 1..self.data.len() {
            let mut new_rays: HashMap<usize, usize> = HashMap::new();

            for (&key, &val) in &rays {
                if self.data[row][key] == '.' {
                    *new_rays.entry(key).or_insert(0) += val;
                } else {
                    // splits += 1;
                    if key > 0 {
                        *new_rays.entry(key - 1).or_insert(0) += val;
                    }
                    if key < self.data[0].len() - 1 {
                        *new_rays.entry(key + 1).or_insert(0) += val;
                    }
                }
            }

            rays = new_rays;
        }
        for (_, val) in rays {
            splits += val;
        }

        splits as i64
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem {
            data: data.iter().map(|line| line.chars().collect()).collect(),
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [21, 40, 1598, 4_509_723_641_302];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/07_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/07_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/07.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/07.txt")).p2(), ANSWERS[3]); }
}

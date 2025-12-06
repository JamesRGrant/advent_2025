use crate::Solve;

pub struct Problem {
    data: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Find the highest 2 digit number in each line and sum them
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let mut max: char = '/';
            let mut max2: char = '/';
            let mut j = 0;

            for (i, x) in line.iter().enumerate().take(line.len() - 1) {
                if *x > max {
                    max = *x;
                    j = i;
                }
            }
            for (_, y) in line.iter().enumerate().skip(j + 1) {
                max2 = max2.max(*y);
            }

            sum += i64::from(max.to_digit(10).unwrap() * 10 + max2.to_digit(10).unwrap());
        }
        sum
    }

    /// Find the highest 12 digit number in each line and sum them
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let line_len = line.len();
            let mut start = 0;
            // Characters left
            for i in (1..=12).rev() {
                let mut max = '/';
                for (j, c) in line.iter().enumerate().skip(start).take(line_len - start - i + 1) {
                    if *c > max {
                        max = *c;
                        start = j + 1;
                    }
                }
                sum += i64::from(max.to_digit(10).unwrap()) * 10i64.pow((i - 1) as u32);
            }
        }

        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut char_data: Vec<Vec<char>> = Vec::with_capacity(data.len());
        for line in data {
            char_data.push(line.chars().collect());
        }
        Problem { data: char_data }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [357, 3_121_910_778_619, 17155, 169_685_670_469_164];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/03_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/03_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/03.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/03.txt")).p2(), ANSWERS[3]); }
}

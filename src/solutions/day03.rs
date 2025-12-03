use crate::Solve;

pub struct Problem {
    data: Vec<String>,
}
impl Solve for Problem {
    /// Short Description
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let mut max: i64 = -1;
            let mut max2: i64 = -1;
            let mut j = 0;

            // Convert to numbers
            let nums: Vec<i64> = line
                .chars()
                .map(|c| i64::from(c.to_digit(10).unwrap()))
                .collect();
            for (i, x) in nums.iter().enumerate().take(nums.len() - 1) {
                if *x > max {
                    max = *x;
                    j = i;
                }
            }
            for (_, y) in nums.iter().enumerate().skip(j + 1) {
                if *y >= max2 {
                    max2 = *y;
                }
            }
            sum += max * 10 + max2;
        }
        sum
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for line in &self.data {
            let mut str = String::new();
            let chars = ['9', '8', '7', '6', '5', '4', '3', '2', '1', '0'];

            let mut next = 0;
            let mut start;
            // Characters left
            'outer: for i in (1..=12).rev() {
                start = next;
                for c in &chars {
                    for j in start..=(line.len() - i) {
                        if line.chars().nth(j).unwrap() == *c {
                            str.push(*c);
                            next = j + 1;
                            continue 'outer;
                        }
                    }
                }
            }

            sum += str.parse::<i64>().unwrap();
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem {
            data: data.to_vec(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [357, 3_121_910_778_619, 17155, 169_685_670_469_164];

    #[test]
    fn p1() {
        assert_eq!(
            Problem::new(&load_file("input/03_test.txt")).p1(),
            ANSWERS[0]
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            Problem::new(&load_file("input/03_test.txt")).p2(),
            ANSWERS[1]
        );
    }
    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/03.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        assert_eq!(Problem::new(&load_file("input/03.txt")).p2(), ANSWERS[3]);
    }
}

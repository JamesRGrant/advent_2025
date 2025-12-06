use crate::Solve;

pub struct Problem {
    numbers: Vec<Vec<i64>>,
    operators: Vec<char>,
    characters: Vec<Vec<char>>,
}
impl Solve for Problem {
    /// Either add or multiply each column of numbers
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (i, op) in self.operators.iter().enumerate() {
            match *op {
                '+' => {
                    for row in &self.numbers {
                        sum += row[i];
                    }
                }
                '*' => {
                    let mut prod = 1;
                    for row in &self.numbers {
                        prod *= row[i];
                    }
                    sum += prod;
                }
                _ => panic!("Invalid operator"),
            }
        }
        sum
    }

    /// Numbers are read top to bottom by char position
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        let mut nums: Vec<i64> = Vec::new();
        let rows = self.characters.len();

        for i in (0..self.characters[0].len()).rev() {
            let mut num_str: String = String::new();
            for j in 0..rows - 1 {
                let c = self.characters[j][i];
                if c.is_ascii_digit() {
                    num_str.push(c);
                }
            }
            if !num_str.is_empty() {
                nums.push(num_str.parse().unwrap());
            }
            let op = self.characters[rows - 1][i];
            match op {
                '+' => {
                    sum += nums.iter().sum::<i64>();
                    nums.clear();
                }
                '*' => {
                    sum += nums.iter().product::<i64>();
                    nums.clear();
                }
                _ => (),
            }
        }
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut numbers: Vec<Vec<i64>> = Vec::new();
        let mut operators: Vec<char> = Vec::new();
        let mut characters: Vec<Vec<char>> = Vec::new();

        // For problem 1, we will parse the numbers and operators separately
        // For problem 2, we will keep the character representation
        for line in data {
            let c = line.chars().next().unwrap();
            if c != '+' && c != '*' {
                let nums: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                numbers.push(nums);
                characters.push(line.chars().collect());
            } else {
                operators = line.split_whitespace().map(|x| x.chars().next().unwrap()).collect();
                characters.push(line.chars().collect());
            }
        }
        Problem {
            numbers,
            operators,
            characters,
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [4_277_556, 3_263_827, 7_326_876_294_741, 10_756_006_415_204];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/06_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/06_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/06.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/06.txt")).p2(), ANSWERS[3]); }
}

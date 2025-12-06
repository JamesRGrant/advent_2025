use crate::Solve;

pub struct Problem {
    direction: Vec<char>,
    number: Vec<i16>,
}
impl Solve for Problem {
    /// Safe combo: how many times it rests at zero
    fn p1(&mut self) -> i64 {
        let mut dial: i16 = 50;
        let mut zeros: i64 = 0;

        // loop through direction and number
        for i in 0..self.direction.len() {
            match self.direction[i] {
                'L' => dial -= self.number[i],
                'R' => dial += self.number[i],
                _ => panic!("Invalid direction"),
            }

            // Remove overflows using modulo
            // Fancy function to get the positive remainder, equiv to ((dial % 100) + 100) % 100;
            dial = dial.rem_euclid(100);

            // Count if at zero
            if dial == 0 {
                zeros += 1;
            }
        }

        zeros
    }

    /// Safe combo: how many times it passes or rests at zero
    fn p2(&mut self) -> i64 {
        #[derive(PartialEq)]
        enum Direction {
            Up,
            Down,
            None,
        }
        let mut dial: i16 = 50;
        let mut zeros: i64 = 0;
        let mut last_direction;
        let mut was_zero = false;

        // Use iterator instead of indexed access for better performance
        for (&direction, &number) in self.direction.iter().zip(self.number.iter()) {
            last_direction = Direction::None;
            match direction {
                'L' => dial -= number,
                'R' => dial += number,
                _ => panic!("Invalid direction"),
            }

            // Handle overflows in chunks of 100
            if dial > 99 {
                zeros += i64::from(dial / 100);
                dial %= 100;
                last_direction = Direction::Up;
            } else if dial < 0 {
                zeros += i64::from((-dial - 1) / 100 + 1);
                // Fancy function to get the positive remainder, equiv to ((dial % 100) + 100) % 100;
                dial = dial.rem_euclid(100);
                last_direction = Direction::Down;
            }

            // Edge case: 0 - 5 is counted above and needs to be rolled back
            if last_direction == Direction::Down && was_zero {
                zeros -= 1;
            }

            if dial == 0 {
                was_zero = true;

                // If up, already counted.  e.g. 50+150 already has 2 counted (200 - 100 - 100)
                match last_direction {
                    Direction::Up => {}
                    Direction::Down | Direction::None => zeros += 1,
                }
            } else {
                was_zero = false;
            }
        }

        zeros
    }
}

impl Problem {
    pub fn new(data: &[String]) -> Self {
        let mut direction = Vec::with_capacity(data.len());
        let mut number = Vec::with_capacity(data.len());

        for line in data {
            direction.push(line.chars().next().unwrap());
            number.push(line[1..].parse().unwrap());
        }

        Problem { direction, number }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [3, 6, 1177, 6768];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/01_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/01_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/01.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/01.txt")).p2(), ANSWERS[3]); }
}

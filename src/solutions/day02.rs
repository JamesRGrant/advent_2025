use crate::Solve;
use std::collections::HashSet;

pub struct Problem {
    data: Vec<(i64, i64)>,
    p2: i64,
}
impl Solve for Problem {
    /// Count repeated halves in ranges, e.g. 123123 is one
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;
        let mut p2: i64 = 0;

        // Find the min/max digit lengths of all the ranges
        let (mut min, mut max) = (usize::MAX, usize::MIN);
        for (start, end) in &self.data {
            min = min.min(start.to_string().len());
            max = max.max(end.to_string().len());
        }

        // Pre-calculate powers for efficiency
        let powers: Vec<i64> = (0..=max * 2)
            .map(|i| 10_i64.pow(u32::try_from(i).unwrap()))
            .collect();

        // Use a set to avoid double counting 22 22 and 2 2 2 2
        // Store sets by length
        let mut splits: Vec<HashSet<i64>> = vec![HashSet::new(); max + 1];
        let mut all: Vec<HashSet<i64>> = vec![HashSet::new(); max + 1];
        for total_len in min..=max {
            for seg_len in 1..=(total_len / 2) {
                if total_len % seg_len == 0 {
                    let segments = total_len / seg_len;

                    // Generate all repeats of this size, e.g. seg_len =2 => 10..=99
                    for seg in powers[seg_len - 1]..powers[seg_len] {
                        let mut num = 0;
                        let mut multiplier = 1;
                        for _ in 0..segments {
                            num += seg * multiplier;
                            multiplier *= powers[seg_len];
                        }
                        if seg_len == total_len / 2 && total_len % 2 == 0 {
                            // Half split for p1
                            splits[total_len].insert(num);
                        }
                        // All repeats for p2
                        all[total_len].insert(num);
                    }
                }
            }
        }
        for (start, end) in &self.data {
            for len in start.to_string().len()..=end.to_string().len() {
                for &num in &splits[len] {
                    if num >= *start && num <= *end {
                        sum += num;
                    }
                }
                for &num in &all[len] {
                    if num >= *start && num <= *end {
                        p2 += num;
                    }
                }
            }
        }
        self.p2 = p2;
        sum
    }

    /// See p1 for details
    fn p2(&mut self) -> i64 {
        assert!(
            self.p2 >= 0,
            "Part 1 must be run before part 2 to set p2 value"
        );
        self.p2
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem {
            data: data[0]
                .split(',')
                .map(|pair| {
                    let mut bounds = pair.split('-');
                    let start: i64 = bounds.next().unwrap().parse().unwrap();
                    let end: i64 = bounds.next().unwrap().parse().unwrap();
                    (start, end)
                })
                .collect(),
            p2: -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [1_227_775_554, 4_174_379_265, 17_077_011_375, 36_037_497_037];

    #[test]
    fn p1() {
        assert_eq!(
            Problem::new(&load_file("input/02_test.txt")).p1(),
            ANSWERS[0]
        );
    }

    #[test]
    fn p2() {
        let mut prob = Problem::new(&load_file("input/02_test.txt"));
        prob.p1(); // Need to run p1 first to set p2 value
        assert_eq!(prob.p2(), ANSWERS[1]);
    }

    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/02.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        let mut prob = Problem::new(&load_file("input/02.txt"));
        prob.p1(); // Need to run p1 first to set p2 value
        assert_eq!(prob.p2(), ANSWERS[3]);
    }
}

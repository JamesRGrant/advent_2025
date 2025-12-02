use crate::Solve;

pub struct Problem {
    data: Vec<(i64, i64)>,
    p2: i64,
}
impl Solve for Problem {
    /// Count repeated halves in ranges, e.g. 123123 is one
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;
        let mut p2: i64 = 0;

        for (start, end) in &self.data {
            let mut walker = *start;
            while walker <= *end {
                let tmp = walker.to_string();
                // Compare halves
                if tmp[0..tmp.len() / 2] == tmp[tmp.len() / 2..] {
                    sum += walker;
                    p2 += walker;
                } else {
                    // Look for more options for part 2
                    // Look for any size repeating multiple times, e.g. 121212
                    // 222222 counts as one, not 3 (2, 22, 222)
                    for size in 1..=(tmp.len() / 2) {
                        // Only split if even segments
                        if tmp.len() % size == 0 {
                            let mut same = true;
                            let mut i = size;
                            let mut j = size * 2;

                            // Check all the segments
                            while j <= tmp.len() {
                                if tmp[0..size] != tmp[i..j] {
                                    same = false;
                                    break;
                                }
                                i += size;
                                j += size;
                            }

                            // Count only once per number
                            if same {
                                p2 += walker;
                                break;
                            }
                        }
                    }
                }

                walker += 1;
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

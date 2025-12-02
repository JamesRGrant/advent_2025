use crate::Solve;

pub struct Problem {
    data: Vec<(i64,i64)>,
}
impl Solve for Problem {
    /// Count repeated halves in ranges, e.g. 123123 is one
    fn p1(&mut self) -> i64 {
        let mut sum: i64 = 0;
        for (start, end) in &self.data {
            let mut walker = *start;
            while walker <= *end {
                let tmp = walker.to_string();
                // Compare halves
                if tmp[0..tmp.len()/2] == tmp[tmp.len()/2..] {
                    sum += walker;
                }
                walker += 1;
            }
        }       
        sum
    }

    /// Look for any size repeating multiple times, e.g. 121212
    /// 222222 counts as one, not 3 (2, 22, 222)
    fn p2(&mut self) -> i64 {
        let mut sum: i64 = 0;
        for (start, end) in &self.data {
            let mut walker = *start;
            while walker <= *end {
                let tmp:String = walker.to_string();

                for size in 1..=(tmp.len()/2) {
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
                            sum += walker;
                            break;
                        }
                    }
                }

                walker += 1;
            }

        }       
        sum
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        Problem { 
            data : data[0].split(',').map(|pair| {
                let mut bounds = pair.split('-');
                let start: i64 = bounds.next().unwrap().parse().unwrap();
                let end: i64 = bounds.next().unwrap().parse().unwrap();
                (start, end)
            }).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [1227775554, 4174379265, 17077011375, 36037497037];

    #[test]
    fn p1() {
        assert_eq!(Problem::new(&load_file("input/02_test.txt")).p1(), ANSWERS[0]);
    }

    #[test]
    fn p2() {
        assert_eq!(Problem::new(&load_file("input/02_test.txt")).p2(), ANSWERS[1]);
    }

    #[test]
    fn f1() {
        assert_eq!(Problem::new(&load_file("input/02.txt")).p1(), ANSWERS[2]);
    }

    #[test]
    fn f2() {
        assert_eq!(Problem::new(&load_file("input/02.txt")).p2(), ANSWERS[3]);
    }
}

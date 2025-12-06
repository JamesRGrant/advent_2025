use crate::Solve;

pub struct Problem {
    ranges: Vec<(i64, i64)>,
    ingredients: Vec<i64>,
}

impl Solve for Problem {
    /// How many ingredients are fresh (in any range)
     #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut count = 0;
        for ingredient in &self.ingredients {
            for (start, end) in &self.ranges {
                if *ingredient >= *start && *ingredient <= *end {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    /// Total list of ingredients that are fresh 
     #[allow(clippy::cast_possible_wrap)]
    fn p2(&mut self) -> i64 {
        self.ranges.iter().map(|(s, e)| e - s + 1).sum::<i64>() 
    }
}
impl Problem {
    // Ranges of fresh ingredients, may overlap, e.g. 3-5
    // Blank line
    // List of ingredients (1 per line)    
    // We are going to immediately merge the overlapping ranges
    pub fn new(data: &[String]) -> Self {
        let mut part1 = true;
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let mut ingredients: Vec<i64> = Vec::new();
        for line in data {
            if line.is_empty() {
                part1 = false;
                continue;
            }
            if part1 {
                // Split line and add to ranges
                let tmp: Vec<&str> = line.split('-').collect();
                Problem::merge(tmp[0].parse().unwrap(), tmp[1].parse().unwrap(), &mut ranges);
            } else {
                // Process part 2 lines
                ingredients.push(line.parse().unwrap());
            }

        }
        Problem { ranges, ingredients}
    }

    fn merge(start: i64, end: i64, collapsed: &mut Vec<(i64,i64)>) {
        let mut new_start = start;
        let mut new_end = end;
        let mut to_remove: Vec<usize> = Vec::new();

        for (i, (c_start, c_end)) in collapsed.iter().enumerate() {
            // Check for overlap
            if !(new_end < *c_start || new_start > *c_end) {
                // Overlap, merge
                new_start = new_start.min(*c_start);
                new_end = new_end.max(*c_end);
                to_remove.push(i);
            }
        }

        // Remove merged ranges
        for &i in to_remove.iter().rev() {
            collapsed.remove(i);
        }

        // Add the new merged range
        collapsed.push((new_start, new_end));
    }
}



#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [3, 14, 520, 347_338_785_050_515];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/05_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/05_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/05.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/05.txt")).p2(), ANSWERS[3]); }
}

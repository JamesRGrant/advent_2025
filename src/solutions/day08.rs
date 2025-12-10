use crate::Solve;
use std::collections::HashSet;

pub struct Problem {
    points: Vec<(i64, i64, i64)>,
    distances: Vec<(usize, usize, i64)>,
}
impl Solve for Problem {
    /// Points in 3D space
    /// Connect N (either 40 or 1000) of the closest points
    /// Find the top 3 longest connections (most points), multiply their lengths
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut chains: Vec<Vec<usize>> = Vec::new();

        let link_count = if self.points.len() == 20 { 10 } else { 1000 };

        for i in 0..link_count {
            let (p1, p2, _dist) = self.distances[i];
            let (mut found_p1, mut found_p2) = (None, None);

            for (chain_idx, chain) in chains.iter_mut().enumerate() {
                if found_p1.is_none() && chain.contains(&p1) {
                    found_p1 = Some(chain_idx);
                }
                if found_p2.is_none() && chain.contains(&p2) {
                    found_p2 = Some(chain_idx);
                }
            }

            // If both found, merge chains
            // If one found, add the other point
            // If none found, create new chain
            if let (Some(c1), Some(c2)) = (found_p1, found_p2) {
                if c1 != c2 {
                    let min = c1.min(c2);
                    let max = c1.max(c2);
                    let mut chains_to_merge = chains.remove(max);
                    chains[min].append(&mut chains_to_merge);
                }
            } else if let Some(c1) = found_p1 {
                chains[c1].push(p2);
            } else if let Some(c2) = found_p2 {
                chains[c2].push(p1);
            } else {
                chains.push(vec![p1, p2]);
            }
        }

        let mut chain_lengths: Vec<i64> = chains.iter().map(|c| c.len() as i64).collect();
        chain_lengths.sort_unstable_by(|a, b| b.cmp(a));
        chain_lengths.truncate(3);
        chain_lengths.iter().product()
    }

    /// Short Description
    fn p2(&mut self) -> i64 {
        let mut chains: Vec<Vec<usize>> = Vec::new();
        let mut points_linked: HashSet<usize> = HashSet::new();
        let mut prod = 0;
        // let mut last_links: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.distances.len() {
            let (p1, p2, _dist) = self.distances[i];
            let (mut found_p1, mut found_p2) = (None, None);

            points_linked.insert(p1);
            points_linked.insert(p2);

            for (chain_idx, chain) in chains.iter_mut().enumerate() {
                if found_p1.is_none() && chain.contains(&p1) {
                    found_p1 = Some(chain_idx);
                }
                if found_p2.is_none() && chain.contains(&p2) {
                    found_p2 = Some(chain_idx);
                }
            }

            // If both found, merge chains
            // If one found, add the other point
            // If none found, create new chain
            if let (Some(c1), Some(c2)) = (found_p1, found_p2) {
                if c1 != c2 {
                    let min = c1.min(c2);
                    let max = c1.max(c2);
                    let mut chains_to_merge = chains.remove(max);
                    chains[min].append(&mut chains_to_merge);
                }
            } else if let Some(c1) = found_p1 {
                chains[c1].push(p2);
            } else if let Some(c2) = found_p2 {
                chains[c2].push(p1);
            } else {
                chains.push(vec![p1, p2]);
            }

            if points_linked.len() == self.points.len() {
                prod = self.points[p1].0 * self.points[p2].0;
                break;
            }
        }

        prod
    }
}
impl Problem {
    pub fn new(data: &[String]) -> Self {
        // Calculate combinations  n! / (k! * (n - k)!)
        // Can be simplifed for k=2 items to n * (n - 1) / 2
        let data_len = data.len();
        let combo_len = data_len * (data_len - 1) / 2;

        let mut points: Vec<(i64, i64, i64)> = Vec::with_capacity(data.len());
        let mut distances: Vec<(usize, usize, i64)> = Vec::with_capacity(combo_len);

        for line in data {
            let mut parts = line.split(',');
            points.push((
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ));
        }

        // Calculate all distances between points
        // Don't square root yet to save time
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let dx = points[i].0 - points[j].0;
                let dy = points[i].1 - points[j].1;
                let dz = points[i].2 - points[j].2;
                let dist = dx * dx + dy * dy + dz * dz;
                distances.push((i, j, dist));
            }
        }

        // Sort distances ascending
        distances.sort_by(|a, b| a.2.cmp(&b.2));

        Problem { points, distances }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [40, 25272, 84968, 8_663_467_782];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/08_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/08_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/08.txt")).p1(), ANSWERS[2]); }
    #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/08.txt")).p2(), ANSWERS[3]); }
}

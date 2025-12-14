use crate::Solve;
use nalgebra::{DMatrix, DVector};
// use std::io::Write;
use std::vec;

pub struct Problem {
    answers: Vec<Vec<bool>>,
    operations: Vec<Vec<Vec<i32>>>,
    joltages: Vec<Vec<i32>>,
}
impl Solve for Problem {
    /// Min operations toggling lights to desired pattern
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (i, answer) in self.answers.iter().enumerate() {
            // Start with [0, 0, ...] and build up to the answer
            let start = vec![false; answer.len()];

            // Store the indexes of remaining operations
            // We will clone and remove these recursively
            let mut remaining_ops = Vec::new();
            for i in 0..self.operations[i].len() {
                remaining_ops.push(i);
            }

            let min = Problem::perform_step(1, answer, &start, &self.operations[i], &remaining_ops);
            assert!(min < 100, "Exceeded step circuit limit of 8");
            sum += min as i64;
        }
        sum
    }

    /// Min operations adding voltages to desired levels
    /// This is a system of linear equations in form of Ax = b
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn p2(&mut self) -> i64 {
        let mut sum = 0;
        for (i, jolts) in self.joltages.iter().enumerate() {
            // Convert to f64 for linear algebra operations
            let b = DVector::from_vec(jolts.iter().map(|&x| f64::from(x)).collect::<Vec<f64>>());

            // Convert instructions to additions, e.g. (0,3) is 1,0,0,1
            let num_equations = jolts.len();
            let num_operations = self.operations[i].len();

            // Create coefficient matrix: each row is an equation, each column is an operation
            let mut jolt_pos = vec![vec![0.0; num_operations]; num_equations];
            for (op_idx, operation) in self.operations[i].iter().enumerate() {
                for &pos in operation {
                    jolt_pos[pos as usize][op_idx] = 1.0;
                }
            }

            let a = DMatrix::from_row_slice(
                num_equations,
                num_operations,
                &jolt_pos.into_iter().flatten().collect::<Vec<f64>>(),
            );

            if num_equations == num_operations {
                println!("Solving square system:");
                if let Some(x) = a.clone().lu().solve(&b) {
                    let rounded: Vec<i64> = x.iter().map(|v| v.round() as i64).collect();
                    println!("Fallback rounded solution: {rounded:?}");
                    sum += rounded.iter().sum::<i64>();
                    continue;
                }
            } else {
                println!("Solving rectangular system:");
            }

            // Find integer solutions

            let integer_solution = Problem::find_integer_solution(&a, &b);
            if let Some(solution) = integer_solution {
                println!("Integer solution: {solution:?}");
                sum += solution.iter().sum::<i64>();
            } else {
                println!("No integer solution found");
                // Fallback to floating point solution rounded
                if num_equations == num_operations {
                    if let Some(x) = a.lu().solve(&b) {
                        let rounded: Vec<i64> = x.iter().map(|v| v.round() as i64).collect();
                        println!("Fallback rounded solution: {rounded:?}");
                        sum += rounded.iter().sum::<i64>();
                    }
                } else {
                    let svd = a.svd(true, true);
                    if let Ok(x) = svd.solve(&b, 1e-10) {
                        let rounded: Vec<i64> = x.iter().map(|v| v.round() as i64).collect();
                        println!("Fallback rounded solution: {rounded:?}");
                        sum += rounded.iter().sum::<i64>();
                    }
                }
            }
        }
        sum
    }
}
impl Problem {
    /// Find integer solution using brute force with bounds
    fn find_integer_solution(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<Vec<i64>> {
        let max_ops_per_variable = 20; // Reasonable upper bound

        // Try to find a solution with minimal sum
        for total_ops in 0..=50 {
            let solutions = Problem::find_integer_solutions_with_sum(a, b, total_ops, max_ops_per_variable);
            if !solutions.is_empty() {
                // Return the first valid solution (could add criteria to pick the best one)
                return Some(solutions[0].clone());
            }
        }
        None
    }

    /// Find all integer solutions that sum to a specific total
    fn find_integer_solutions_with_sum(
        a: &DMatrix<f64>,
        b: &DVector<f64>,
        target_sum: i64,
        max_per_var: i64,
    ) -> Vec<Vec<i64>> {
        let mut solutions = Vec::new();
        let num_ops = a.ncols();

        // Generate all combinations that sum to target_sum
        Problem::generate_combinations(
            num_ops,
            target_sum,
            max_per_var,
            &mut vec![0; num_ops],
            0,
            &mut solutions,
            a,
            b,
        );
        solutions
    }

    /// Recursively generate integer combinations
    #[allow(clippy::too_many_arguments)]
    fn generate_combinations(
        num_vars: usize,
        remaining_sum: i64,
        max_per_var: i64,
        current: &mut Vec<i64>,
        index: usize,
        solutions: &mut Vec<Vec<i64>>,
        a: &DMatrix<f64>,
        b: &DVector<f64>,
    ) {
        if index == num_vars {
            if remaining_sum == 0 && Problem::verify_solution(a, b, current) {
                solutions.push(current.clone());
            }
            return;
        }

        for value in 0..=std::cmp::min(remaining_sum, max_per_var) {
            current[index] = value;
            Problem::generate_combinations(
                num_vars,
                remaining_sum - value,
                max_per_var,
                current,
                index + 1,
                solutions,
                a,
                b,
            );
        }
    }

    /// Verify that Ax = b for integer solution
    #[allow(clippy::cast_precision_loss)]
    fn verify_solution(a: &DMatrix<f64>, b: &DVector<f64>, x: &[i64]) -> bool {
        let tolerance = 1e-10;

        for i in 0..a.nrows() {
            let mut result = 0.0;
            for j in 0..a.ncols() {
                result += a[(i, j)] * x[j] as f64;
            }
            if (result - b[i]).abs() > tolerance {
                return false;
            }
        }
        true
    }

    fn perform_step(step: usize, answer: &[bool], input: &[bool], ops: &[Vec<i32>], remaining_ops: &[usize]) -> usize {
        let mut min_step = usize::MAX;

        if step > 8 {
            return step * 100;
        }
        for i in 0..remaining_ops.len() {
            let mut reduced_ops = remaining_ops.to_vec();
            let op_idx = reduced_ops.remove(i);
            let calc = Problem::perform_op(input, &ops[op_idx]);
            if calc == *answer {
                // Found it!
                return step;
            }
            let this_step = Problem::perform_step(step + 1, answer, &calc, ops, &reduced_ops);
            if this_step == step + 1 {
                // Early exit if we found the best solution
                return step + 1;
            }
            min_step = min_step.min(this_step);
        }

        min_step
    }

    #[allow(clippy::cast_sign_loss)]
    fn perform_op(input: &[bool], op: &Vec<i32>) -> Vec<bool> {
        let mut answer = input.to_vec();
        for flip in op {
            answer[*flip as usize] = !answer[*flip as usize];
        }
        answer
    }

    // Format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    pub fn new(data: &[String]) -> Self {
        let mut answers: Vec<Vec<bool>> = Vec::with_capacity(data.len());
        let mut operations: Vec<Vec<Vec<i32>>> = Vec::with_capacity(data.len());
        let mut joltages: Vec<Vec<i32>> = Vec::with_capacity(data.len());
        for line in data {
            let mut parts = line.split(' ');

            // Parse the answer in position one
            let mut first = parts.next().unwrap().chars().collect::<Vec<char>>();
            first.pop();
            first.remove(0);
            let first_bools = first.iter().map(|&c| c == '#').collect::<Vec<bool>>();
            answers.push(first_bools);

            // Parse the operations (all but last)
            let mut line_operations: Vec<Vec<i32>> = Vec::new();
            for _ in 0..parts.clone().count() - 1 {
                let op = parts.next().unwrap().replace(['(', ')'], "");
                // split and parse into ints
                let op_elements = op.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                line_operations.push(op_elements);
            }
            operations.push(line_operations);

            // Parse the last section for p2 later
            // Parse the answer in position one
            let last_nums = parts
                .next_back()
                .unwrap()
                .replace(['{', '}'], "")
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            joltages.push(last_nums);
        }
        Problem {
            answers,
            operations,
            joltages,
        }
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod test {
    use super::*;
    use crate::load_file;
    const ANSWERS: [i64; 4] = [7, 33, 415, 0];

    #[test] fn p1() { assert_eq!(Problem::new(&load_file("input/10_test.txt")).p1(), ANSWERS[0]); }
    #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/10_test.txt")).p2(), ANSWERS[1]); }
    // #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/10.txt")).p1(), ANSWERS[2]); }
    // #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/10.txt")).p2(), ANSWERS[3]); }
}

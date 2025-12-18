use crate::Solve;
// use good_lp::*;
// use nalgebra::{DMatrix, DVector};
// use std::io::Write;

pub struct Problem {
    lights: Vec<u16>,
    binary_ops: Vec<Vec<u16>>,
    operations: Vec<Vec<Vec<i32>>>,
    joltages: Vec<Vec<i32>>,
}
impl Solve for Problem {
    /// Min operations toggling lights to desired pattern
    /// Using binary representation and just brute forcing all combinations
    #[allow(clippy::cast_possible_wrap)]
    fn p1(&mut self) -> i64 {
        let mut sum = 0;
        for (i, answer) in self.lights.iter().enumerate() {
            let mut bmin = i64::MAX;

            // Try all combinations. e.g. for 2 operations: 01, 10, 11, which is 1..4
            let num_combo = 2_u32.pow(u32::try_from(self.operations[i].len()).unwrap());
            for check in 1..num_combo {
                let mut steps: i64 = 0;
                let mut b_start: u16 = 0;

                for x in 0..self.operations[i].len() {
                    // If bit i is set, we apply that operation
                    if (check & (1 << x)) != 0 {
                        b_start ^= self.binary_ops[i][x];
                        steps += 1;
                    }
                }
                if b_start == *answer {
                    bmin = bmin.min(steps);
                    if bmin == 1 {
                        break;
                    }
                }
            }

            sum += bmin;
        }
        sum
    }

    /// Min operations adding voltages to desired levels
    /// This is a system of linear equations in form of Ax = b
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    fn p2(&mut self) -> i64 {
        // let mut sum = 0;
        // for (i, jolts) in self.joltages.iter().enumerate() {
        //     let num_answers = jolts.len();
        //     let num_operations = self.operations[i].len();

        //     // Convert instructions full array, e.g. (0,3) is 1,0,0,1
        //     // Each row is an operation, each column is a voltage position
        //     let mut jolt_pos = vec![vec![0_f64; num_answers]; num_operations];
        //     for (op_idx, operation) in self.operations[i].iter().enumerate() {
        //         for &pos in operation {
        //             jolt_pos[op_idx][pos as usize] = 1.0;
        //         }
        //     }

        //     // Transpose so each row is a voltage position constraint
        //     // and each column represents an operation coefficient
        //     let mut transposed = vec![vec![0.0_f64; num_operations]; num_answers];
        //     for op in 0..num_operations {
        //         for pos in 0..num_answers {
        //             transposed[pos][op] = jolt_pos[op][pos];
        //         }
        //     }

        //     let rows = num_answers; // number of voltage constraints
        //     let cols = num_operations; // number of operation coefficients to solve for

        //     // Create matrixes A and b (convert to f64 for linear algebra)
        //     let matrix_data: Vec<f64> = transposed.concat();
        //     let jolts_f64: Vec<f64> = jolts.iter().map(|&x| x as f64).collect();
        //     let a_matrix = DMatrix::from_row_slice(rows, cols, &matrix_data);
        //     let b_matrix = DVector::from_row_slice(&jolts_f64);

        //     // Solve it with SVD
        //     let solution = a_matrix.svd(true, true).solve(&b_matrix, 1.0e-8);

        //     // Sum the values in solution
        //     if let Ok(sol) = solution {
        //         // Debug: print the actual solution values
        //         println!("Solution values: {:?}", sol.as_slice());

        //         // Better rounding logic based on the actual values we're seeing
        //         let val = sol
        //             .iter()
        //             .map(|&x| {
        //                 let abs_x = x.abs();
        //                 if abs_x < 0.1 {
        //                     0_i64
        //                 } else if abs_x < 1.5 {
        //                     1_i64
        //                 } else {
        //                     abs_x.round() as i64
        //                 }
        //             })
        //             .sum::<i64>();

        //         sum += val;
        //         print!("{val}.");
        //     } else {
        //         print!("X.");
        //     }
        //     std::io::stdout().flush().unwrap();
        // }
        // println!();
        // sum
        0
    }
}
impl Problem {
    // fn solve_with_milp(coefficients: &[i32], target_sum: i32, num_vars: usize) -> Option<Vec<i32>> {
    //     // Create a new problem
    //     let mut problem = ProblemVariables::new();

    //     // Create integer variables (0 or 1 for binary, or bounded integers)
    //     let vars: Vec<Variable> = (0..coefficients.len())
    //         .map(|_| problem.add(variable().integer().bounds(0..=1)))
    //         .collect();

    //     // Constraint: exactly num_vars variables should be 1
    //     let selection_constraint = vars
    //         .iter()
    //         .fold(Expression::from(0), |acc, &var| acc + var)
    //         .eq(num_vars as i32);

    //     // Constraint: sum of selected coefficients equals target
    //     let sum_constraint = coefficients
    //         .iter()
    //         .zip(vars.iter())
    //         .fold(Expression::from(0), |acc, (&coeff, &var)| acc + coeff * var)
    //         .eq(target_sum);

    //     // Create the problem
    //     let solution = problem
    //         .minimise(Expression::from(0)) // We just want feasibility, not optimization
    //         .using(default_solver)
    //         .with(selection_constraint)
    //         .with(sum_constraint)
    //         .solve();

    //     match solution {
    //         Ok(sol) => {
    //             let result: Vec<i32> = vars.iter().map(|&var| sol.value(var).round() as i32).collect();
    //             Some(result)
    //         }
    //         Err(_) => None,
    //     }
    // }

    // // For your specific use case - finding which items to select
    // fn solve_subset_sum(values: &[i32], target: i32, count: usize) -> Option<Vec<usize>> {
    //     let mut problem = ProblemVariables::new();

    //     // Binary variables: 1 if item i is selected, 0 otherwise
    //     let vars: Vec<Variable> = (0..values.len()).map(|_| problem.add(variable().binary())).collect();

    //     let solution = problem
    //         .minimise(Expression::from(0))
    //         .using(default_solver)
    //         // Exactly 'count' items selected
    //         .with(
    //             vars.iter()
    //                 .fold(Expression::from(0), |acc, &var| acc + var)
    //                 .eq(count as i32),
    //         )
    //         // Sum equals target
    //         .with(
    //             values
    //                 .iter()
    //                 .zip(vars.iter())
    //                 .fold(Expression::from(0), |acc, (&val, &var)| acc + val * var)
    //                 .eq(target),
    //         )
    //         .solve();

    //     match solution {
    //         Ok(sol) => {
    //             let selected_indices: Vec<usize> = vars
    //                 .iter()
    //                 .enumerate()
    //                 .filter_map(
    //                     |(i, &var)| {
    //                         if sol.value(var).round() as i32 == 1 {
    //                             Some(i)
    //                         } else {
    //                             None
    //                         }
    //                     },
    //                 )
    //                 .collect();
    //             Some(selected_indices)
    //         }
    //         Err(_) => None,
    //     }
    // }

    // // Example usage replacing your current brute force approach
    // fn solve_column_milp(voltages: &[i32], target: i32, num_vars: usize) -> Option<Vec<usize>> {
    //     Problem::solve_subset_sum(voltages, target, num_vars)
    // }

    // #[allow(clippy::cast_sign_loss)]
    // fn solve_system(ops: &[Vec<i32>], jolts: &[i32]) -> i64 {
    //     let mut min = i64::MAX;
    //     let min_col = Problem::min_col(jolts);
    //     assert!(min_col != usize::MAX);
    //     let var_count = Problem::sum_col(ops, min_col);
    //     assert!(var_count > 0);
    //     // Create the possibilites to go through for this equation
    //     let mut combos = Problem::generate_combinations(var_count as usize, i64::from(jolts[min_col]));
    //     println!(
    //         "Solving for col {min_col}: {jolts:?} needing {} vars to sum to {} => {} combos",
    //         var_count,
    //         jolts[min_col],
    //         combos.len()
    //     );

    //     for combo in &mut combos {
    //         // print!("{jolts:?} Trying combo {:?} => ", combo);
    //         let mut test_jolts = jolts.to_vec();
    //         let mut op_count = 0;
    //         for operator in ops {
    //             if operator[min_col] == 1 {
    //                 let c = combo.remove(0);
    //                 op_count += c;

    //                 for (i, flag) in operator.iter().enumerate() {
    //                     if *flag == 1 {
    //                         test_jolts[i] -= i32::try_from(c).unwrap();
    //                     }
    //                 }
    //             }
    //         }
    //         assert!(test_jolts[min_col] == 0);
    //         // println!("gives {:?}", test_jolts);

    //         if Problem::is_zero(&test_jolts) {
    //             println!("  Found solution for {op_count}");
    //             min = min.min(op_count);
    //         } else if !Problem::is_neg(&test_jolts) {
    //             let mut remaining_ops = Vec::new();
    //             for op in ops {
    //                 if op[min_col] == 0 {
    //                     remaining_ops.push(op.clone());
    //                 }
    //             }
    //             if remaining_ops.is_empty() {
    //                 continue;
    //             }
    //             let recurse_min = Problem::solve_system(ops, &test_jolts);
    //             if recurse_min != i64::MAX {
    //                 min = min.min(op_count + recurse_min);
    //             }
    //         }
    //     }
    //     // println!("Pos {min_col} with {var_count}, min = {min}");

    //     // println!("  --> {min} for {jolts:?}");

    //     min
    // }

    // fn min_col(val: &[i32]) -> usize {
    //     let mut min_idx = usize::MAX;
    //     let mut min_val = i32::MAX;
    //     for (i, &v) in val.iter().enumerate() {
    //         if v < min_val && v > 0 {
    //             min_val = v;
    //             min_idx = i;
    //         }
    //     }
    //     min_idx
    // }

    // fn sum_col(m: &[Vec<i32>], col: usize) -> i32 {
    //     let mut sum = 0;
    //     for row in m {
    //         sum += row[col];
    //     }
    //     sum
    // }

    // /// Generate all integer combinations that sum to target using iterative approach
    // fn generate_combinations(num_vars: usize, target_sum: i64) -> Vec<Vec<i64>> {
    //     let mut results = Vec::new();

    //     // Counter-based approach (like an odometer)
    //     let mut combination = vec![0_i64; num_vars];

    //     loop {
    //         // Check if current combination sums to target
    //         if combination.iter().sum::<i64>() == target_sum {
    //             results.push(combination.clone());
    //         }

    //         // Increment combination like an odometer
    //         let mut carry = 1;
    //         for c in combination.iter_mut().take(num_vars) {
    //             *c += carry;
    //             if *c <= target_sum {
    //                 carry = 0;
    //                 break;
    //             }
    //             *c = 0;
    //         }

    //         // If we have carry left, we've exhausted all combinations
    //         if carry == 1 {
    //             break;
    //         }
    //     }

    //     results
    // }

    // fn is_zero(v: &[i32]) -> bool {
    //     for &val in v {
    //         if val != 0 {
    //             return false;
    //         }
    //     }
    //     true
    // }

    // fn is_neg(v: &[i32]) -> bool {
    //     for &val in v {
    //         if val < 0 {
    //             return true;
    //         }
    //     }
    //     false
    // }

    // Format: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // Max length of the positions is 10
    // Max count of the operations is 13
    pub fn new(data: &[String]) -> Self {
        let mut answers: Vec<Vec<bool>> = Vec::with_capacity(data.len());
        let mut operations: Vec<Vec<Vec<i32>>> = Vec::with_capacity(data.len());
        let mut joltages: Vec<Vec<i32>> = Vec::with_capacity(data.len());
        let mut lights: Vec<u16> = Vec::with_capacity(data.len());
        let mut binary_ops: Vec<Vec<u16>> = Vec::with_capacity(data.len());

        for line in data {
            let mut parts = line.split(' ');

            // Parse the answer in position one
            let mut first = parts.next().unwrap().chars().collect::<Vec<char>>();
            first.pop();
            first.remove(0);
            let first_bools = first.iter().map(|&c| c == '#').collect::<Vec<bool>>();

            // Convert to a bit mapping
            let mut light: u16 = 0;
            for (i, val) in first_bools.iter().enumerate() {
                if i > 0 {
                    // Shift left
                    light <<= 1;
                }
                if *val {
                    light += 1;
                }
            }
            let light_len = first_bools.len();
            lights.push(light);
            answers.push(first_bools);

            // Parse the operations (all but last)
            let mut line_operations: Vec<Vec<i32>> = Vec::new();
            let mut binary_group: Vec<u16> = Vec::new();
            for _ in 0..parts.clone().count() - 1 {
                let op = parts.next().unwrap().replace(['(', ')'], "");
                // split and parse into ints
                let op_elements = op.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

                // bindary versions
                let mut binary_op: u16 = 0;
                for &pos in &op_elements {
                    // set the bit at position pos
                    // the position varies on the length of the light
                    // e.g. 0 for length 4
                    binary_op |= 1 << (light_len - 1 - pos as usize);
                }
                binary_group.push(binary_op);

                line_operations.push(op_elements);
            }
            binary_ops.push(binary_group);
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
            lights,
            binary_ops,
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
    // #[test] fn p2() { assert_eq!(Problem::new(&load_file("input/10_test.txt")).p2(), ANSWERS[1]); }
    #[test] fn f1() { assert_eq!(Problem::new(&load_file("input/10.txt")).p1(), ANSWERS[2]); }
    // #[test] fn f2() { assert_eq!(Problem::new(&load_file("input/10.txt")).p2(), ANSWERS[3]); }
}

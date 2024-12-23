use crate::utils;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn parse(input: String) -> (u64, Vec<u64>) {
    let mut iter = input.split(':');

    let desired_result = iter
        .next()
        .unwrap()
        .parse()
        .expect("Could not parse u64 desired_result");

    let operands = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| {
            x.parse()
                .expect(format!("Could not parse u64 operand {}", x).as_str())
        })
        .collect();

    (desired_result, operands)
}

fn result2(operation: &Operation, a: u64, b: u64) -> u64 {
    match operation {
        Operation::Add => a + b,
        Operation::Multiply => a * b,
        Operation::Concatenate => a * 10_u64.pow(utils::digit_count(b)) + b,
    }
}

fn result(operations: &[Operation], operands: &Vec<u64>) -> u64 {
    if operations.len() + 1 != operands.len() {
        panic!("Invalid number of operations");
    }

    let first_two_result = result2(&operations[0], operands[0], operands[1]);
    if operands.len() == 2 {
        first_two_result
    } else {
        let mut new_vec = vec![first_two_result];
        new_vec.extend_from_slice(&operands[2..]);

        result(&operations[1..], &new_vec)
    }
}

fn generate_combinations(n: usize, use_concatenation: bool) -> Vec<Vec<Operation>> {
    match (n, use_concatenation) {
        (1, false) => vec![vec![Operation::Add], vec![Operation::Multiply]],
        (1, true) => vec![
            vec![Operation::Add],
            vec![Operation::Multiply],
            vec![Operation::Concatenate],
        ],
        _ => {
            let mut result = Vec::new();
            generate_combinations(n - 1, use_concatenation)
                .iter_mut()
                .for_each(|subcombo| {
                    let (mut with_add, mut with_mul) = (subcombo.clone(), subcombo.clone());
                    with_add.push(Operation::Add);
                    with_mul.push(Operation::Multiply);
                    result.push(with_add);
                    result.push(with_mul);
                    if use_concatenation {
                        let mut with_concat = subcombo.clone();
                        with_concat.push(Operation::Concatenate);
                        result.push(with_concat);
                    }
                });
            result
        }
    }
}

fn has_solution(desired_result: &u64, operands: &Vec<u64>, use_concatenation: bool) -> bool {
    match operands.len() {
        n if n < 2 => false,
        _ => {
            let operation_combinations =
                generate_combinations(operands.len() - 1, use_concatenation);
            for operations in operation_combinations {
                if *desired_result == result(&operations, &operands) {
                    // println!(
                    //     "Found solution: {} = {:?} using operations: {:?}",
                    //     desired_result, operands, operations
                    // );
                    return true;
                }
            }
            false
        }
    }
}

pub fn d7p1(file_path: &str) -> u64 {
    utils::string_iter(file_path)
        .map(parse)
        .filter(|(desired, operands)| has_solution(desired, operands, false))
        .map(|(desired, _)| desired)
        .sum()
}

pub fn d7p2(file_path: &str) -> u64 {
    utils::string_iter(file_path)
        .map(parse)
        .filter(|(desired, operands)| has_solution(desired, operands, true))
        .map(|(desired, _)| desired)
        .sum()
}

pub fn d7() {
    //let file_path = "inputs/d7sample.txt";
    let file_path = "inputs/d7.txt";
    let mut result = d7p1(file_path);
    println!("Result Day 7 Part 1: {}", result);
    result = d7p2(file_path);
    println!("Result Day 7 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_2_test() {
        assert_eq!(result2(&Operation::Add, 2, 3), 5);
        assert_eq!(result2(&Operation::Multiply, 2, 3), 6);
        assert_eq!(result2(&Operation::Concatenate, 2, 3), 23);
        assert_eq!(result2(&Operation::Concatenate, 123, 456), 123456);
    }

    #[test]
    fn gen_combos_test() {
        assert_eq!(
            generate_combinations(1, false),
            vec![vec![Operation::Add], vec![Operation::Multiply]]
        );
        assert_eq!(
            generate_combinations(2, false).sort(),
            vec![
                vec![Operation::Add, Operation::Add],
                vec![Operation::Add, Operation::Multiply],
                vec![Operation::Multiply, Operation::Add],
                vec![Operation::Multiply, Operation::Multiply],
            ]
            .sort()
        );
    }

    #[test]
    fn result_test() {
        assert_eq!(result(&vec![Operation::Add], &vec![2, 3]), 2 + 3);
        assert_eq!(result(&vec![Operation::Multiply], &vec![2, 3]), 2 * 3);

        assert_eq!(
            result(&vec![Operation::Add, Operation::Add], &vec![2, 3, 4]),
            2 + 3 + 4
        );
        assert_eq!(
            result(&vec![Operation::Add, Operation::Multiply], &vec![2, 3, 4]),
            (2 + 3) * 4
        );
        assert_eq!(
            result(&vec![Operation::Multiply, Operation::Add], &vec![2, 3, 4]),
            2 * 3 + 4
        );
        assert_eq!(
            result(
                &vec![Operation::Multiply, Operation::Multiply],
                &vec![2, 3, 4]
            ),
            2 * 3 * 4
        );

        assert_eq!(
            result(
                &vec![Operation::Concatenate, Operation::Concatenate],
                &vec![2, 3, 4]
            ),
            234
        );
        assert_eq!(
            result(
                &vec![Operation::Add, Operation::Concatenate],
                &vec![2, 3, 4]
            ),
            54
        );
        assert_eq!(
            result(
                &vec![Operation::Concatenate, Operation::Add],
                &vec![2, 3, 4]
            ),
            23 + 4
        );

        assert_eq!(
            result(
                &vec![Operation::Add, Operation::Add, Operation::Add],
                &vec![2, 3, 4, 5]
            ),
            2 + 3 + 4 + 5
        );
        assert_eq!(
            result(
                &vec![Operation::Multiply, Operation::Add, Operation::Multiply],
                &vec![2, 3, 4, 5]
            ),
            (2 * 3 + 4) * 5
        );
        assert_eq!(
            result(
                &vec![
                    Operation::Multiply,
                    Operation::Multiply,
                    Operation::Multiply
                ],
                &vec![2, 3, 4, 5]
            ),
            2 * 3 * 4 * 5
        );
    }
}

use crate::utils;

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

// for operands.len() = 3, combinations are 00 (a+b+c), 01 (a+b*c), 10 (a*b+c), 11 (a*b*c)
fn result2(combination: u32, operands: &Vec<u64>) -> u64 {
    if combination == 0 {
        operands[0] + operands[1]
    } else {
        operands[0] * operands[1]
    }
}

fn result(combination: u32, operands: &Vec<u64>) -> u64 {
    if operands.len() == 2 {
        result2(combination, operands)
    } else {
        let bit_count = operands.len() - 1;
        let high_bit_mask = 2_u32.pow(bit_count as u32 - 1);
        let low_bit_mask = high_bit_mask - 1;
        let high_bit = (combination & high_bit_mask) >> (bit_count - 1);
        let low_bits = combination & low_bit_mask;
        let first_two_result = if high_bit == 0 {
            operands[0] + operands[1]
        } else {
            operands[0] * operands[1]
        };

        let mut new_vec = vec![first_two_result];
        new_vec.extend_from_slice(&operands[2..]);

        result(low_bits, &new_vec)
    }
}

fn has_solution((desired_result, operands): &(u64, Vec<u64>)) -> bool {
    match operands.len() {
        n if n < 2 => false,
        //        2 => has_solution_2(desired_result, operands[0], operands[1]),
        _ => {
            let combinations = 2_u32.pow(operands.len() as u32 - 1);
            for combo in 0..combinations {
                if *desired_result == result(combo, &operands) {
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
        .filter(has_solution)
        .map(|(desired, _)| desired)
        .sum()
}

pub fn d7p2(file_path: &str) -> u64 {
    0
}

pub fn d7() {
    //let file_path = "inputs/d7sample.txt";
    let file_path = "inputs/d7.txt";
    let mut result = d7p1(file_path);
    println!("Result Day 7 Part 1: {}", result);
    result = d7p2(file_path);
    println!("Result Day 7 Part 2: {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn result_2_test() {
        assert_eq!(result2(0, &vec![2, 3]), 5);
        assert_eq!(result2(1, &vec![2, 3]), 6);
    }

    #[test]
    fn result_test() {
        assert_eq!(result(0, &vec![2, 3]), 2 + 3);
        assert_eq!(result(1, &vec![2, 3]), 2 * 3);
        assert_eq!(result(0b00, &vec![2, 3, 4]), 2 + 3 + 4);
        assert_eq!(result(0b01, &vec![2, 3, 4]), (2 + 3) * 4);
        assert_eq!(result(0b10, &vec![2, 3, 4]), 2 * 3 + 4);
        assert_eq!(result(0b11, &vec![2, 3, 4]), 2 * 3 * 4);
        assert_eq!(result(0b000, &vec![2, 3, 4, 5]), 2 + 3 + 4 + 5);
        assert_eq!(result(0b101, &vec![2, 3, 4, 5]), (2 * 3 + 4) * 5);
        assert_eq!(result(0b111, &vec![2, 3, 4, 5]), 2 * 3 * 4 * 5);
    }
}

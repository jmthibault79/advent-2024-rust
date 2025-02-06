#[derive(Clone, PartialEq, Debug)]
struct State {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    inst_ptr: usize,
    output: Vec<u8>,
}

enum OpCode {
    Adv,
    Bdv,
    Cdv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
}

fn to_opcode(code: u8) -> OpCode {
    match code {
        0 => OpCode::Adv,
        1 => OpCode::Bxl,
        2 => OpCode::Bst,
        3 => OpCode::Jnz,
        4 => OpCode::Bxc,
        5 => OpCode::Out,
        6 => OpCode::Bdv,
        7 => OpCode::Cdv,
        _ => panic!("Invalid opcode"),
    }
}

// some operands are "combo" operands
fn combo(state: &State, operand: u8) -> usize {
    match operand {
        0..=3 => operand as usize,
        4 => state.reg_a,
        5 => state.reg_b,
        6 => state.reg_c,
        _ => panic!("Invalid combo operand"),
    }
}

// integer division: A = A / 2^combo(operand)
fn op_adv(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.reg_a = state.reg_a / (2 as usize).pow(combo(&state, operand) as u32);
    new_state
}

// integer division: B = A / 2^combo(operand)
fn op_bdv(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.reg_b = state.reg_a / (2 as usize).pow(combo(&state, operand) as u32);
    new_state
}

// integer division: C = A / 2^combo(operand)
fn op_cdv(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.reg_c = state.reg_a / (2 as usize).pow(combo(&state, operand) as u32);
    new_state
}

// B = bitwise xor of B and operand
fn op_bxl(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.reg_b = state.reg_b ^ operand as usize;
    new_state
}

// B = combo operand mod 8
fn op_bst(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.reg_b = combo(&state, operand) % 8;
    new_state
}

// jump to operand if A is not zero
fn op_jnz(state: &State, operand: u8) -> State {
    if state.reg_a == 0 {
        state.clone()
    } else {
        let mut new_state = state.clone();
        new_state.inst_ptr = operand as usize;
        return new_state;
    }
}

// B = B xor C
fn op_bxc(state: &State) -> State {
    let mut new_state = state.clone();
    new_state.reg_b = state.reg_b ^ state.reg_c;
    new_state
}

// add combo operand mod 8 to output
fn op_out(state: &State, operand: u8) -> State {
    let mut new_state = state.clone();
    new_state.output.push(combo(&state, operand) as u8 % 8);
    new_state
}

fn op(state: &State, opcode: u8, operand: u8) -> State {
    let mut new_state = match to_opcode(opcode) {
        OpCode::Adv => op_adv(state, operand),
        OpCode::Bdv => op_bdv(state, operand),
        OpCode::Cdv => op_cdv(state, operand),
        OpCode::Bxl => op_bxl(state, operand),
        OpCode::Bst => op_bst(state, operand),
        OpCode::Jnz => op_jnz(state, operand),
        OpCode::Bxc => op_bxc(state),
        OpCode::Out => op_out(state, operand),
    };

    // if no jump
    if state.inst_ptr == new_state.inst_ptr {
        new_state.inst_ptr += 2;
    }

    // println!(
    //     "op({:?}, {}, {}) -> {:?}",
    //     state, opcode, operand, new_state
    // );

    new_state
}

fn program(mut state: State, program: Vec<u8>) -> State {
    while state.inst_ptr < program.len() {
        let (opcode, operand) = (program[state.inst_ptr], program[state.inst_ptr + 1]);
        state = op(&state, opcode, operand);
    }
    state
}

// manually parsed
fn d17_input() -> (State, Vec<u8>) {
    let state = State {
        reg_a: 62769524,
        reg_b: 0,
        reg_c: 0,
        inst_ptr: 0,
        output: vec![],
    };
    let prog = vec![2, 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0];
    (state, prog)
}

pub fn d17p1() -> String {
    let (start_state, prog) = d17_input();
    program(start_state, prog)
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn p2_out(start_state: &State, a: usize, prog: Vec<u8>) -> String {
    let out = program(start_state.clone(), prog)
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    vec![a.to_string(), "->".to_string(), out].join(" ")
}

fn p2_manual_testing() {
    let (mut start_state, desired_prog) = d17_input();

    println!(
        "{}",
        desired_prog
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );

    // 0
    start_state.reg_a = 7;
    let mut out = p2_out(&start_state, start_state.reg_a, desired_prog.clone());
    println!("{}", out);

    // 3,0
    start_state.reg_a = start_state.reg_a * 8 + 2;
    out = p2_out(&start_state, start_state.reg_a, desired_prog.clone());
    println!("{}", out);

    // 5,3,0
    start_state.reg_a = start_state.reg_a * 8 + 6;
    out = p2_out(&start_state, start_state.reg_a, desired_prog.clone());
    println!("{}", out);
}

fn match_program(start_state: &State, desired_prog: &Vec<u8>, prog_to_match: &[u8]) -> usize {
    let mut a = if prog_to_match.len() > 1 {
        8 * match_program(start_state, desired_prog, &prog_to_match[1..])
    } else {
        1
    };
    let mut state = start_state.clone();
    loop {
        state.reg_a = a;
        let out = program(state.clone(), desired_prog.clone()).output;
        if out == *prog_to_match {
            println!(
                "Matched {} -> {}",
                a,
                out.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
            break;
        }
        a += 1;
    }
    a
}

pub fn d17p2() -> String {
    let (start_state, desired_prog) = d17_input();

    let a = match_program(&start_state, &desired_prog, &desired_prog);
    a.to_string()
}

pub fn d17() {
    let mut result = d17p1();
    println!("Result Day X Part 1: {:?}", result);
    result = d17p2();
    println!("Result Day X Part 2: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut state = State {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            inst_ptr: 0,
            output: vec![],
        };
        let mut expected_state = state.clone();
        expected_state.reg_b = 1;
        expected_state.inst_ptr = 2;
        assert_eq!(op(&state, 2, 6), expected_state);

        // If register B contains 29, the program 1,7 would set register B to 26.
        state = State {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            inst_ptr: 0,
            output: vec![],
        };
        expected_state = state.clone();
        expected_state.reg_b = 26;
        expected_state.inst_ptr = 2;
        assert_eq!(op(&state, 1, 7), expected_state);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        state = State {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            inst_ptr: 0,
            output: vec![],
        };
        expected_state = state.clone();
        expected_state.reg_b = 44354;
        expected_state.inst_ptr = 2;
        assert_eq!(op(&state, 4, 0), expected_state);
    }

    #[test]
    fn test_program_single_op() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut state = State {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            inst_ptr: 0,
            output: vec![],
        };
        let mut expected_state = state.clone();
        expected_state.reg_b = 1;
        expected_state.inst_ptr = 2;
        assert_eq!(program(state, vec![2, 6]), expected_state);

        // If register B contains 29, the program 1,7 would set register B to 26.
        state = State {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            inst_ptr: 0,
            output: vec![],
        };
        expected_state = state.clone();
        expected_state.reg_b = 26;
        expected_state.inst_ptr = 2;
        assert_eq!(program(state, vec![1, 7]), expected_state);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        state = State {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            inst_ptr: 0,
            output: vec![],
        };
        expected_state = state.clone();
        expected_state.reg_b = 44354;
        expected_state.inst_ptr = 2;
        assert_eq!(program(state, vec![4, 0]), expected_state);
    }

    #[test]
    fn test_program() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut state = State {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            inst_ptr: 0,
            output: vec![],
        };
        let mut expected_output = vec![0, 1, 2];
        let final_state = program(state, vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(final_state.output, expected_output);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        state = State {
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            inst_ptr: 0,
            output: vec![],
        };
        expected_output = vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0];
        let final_state = program(state, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(final_state.output, expected_output);
        assert_eq!(final_state.reg_a, 0);

        state = State {
            reg_a: 729,
            reg_b: 0,
            reg_c: 0,
            inst_ptr: 0,
            output: vec![],
        };
        let expected_output = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];
        let final_state = program(state, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(final_state.output, expected_output);
    }

    #[test]
    fn test_match_program() {
        let (start_state, desired_prog) = d17_input();
        assert_eq!(match_program(&start_state, &desired_prog, &[0]), 7);
        assert_eq!(
            match_program(&start_state, &desired_prog, &[3, 0]),
            7 * 8 + 2
        );
        assert_eq!(
            match_program(&start_state, &desired_prog, &[5, 3, 0]),
            (7 * 8 + 2) * 8 + 6
        );
    }
}

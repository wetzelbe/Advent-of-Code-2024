
pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

fn parse_input(path: std::path::PathBuf) -> Computer {
    let mut a: u128 = 0;
    let mut b: u128 = 0;
    let mut c: u128 = 0;
    let mut program = vec![];


    let content = std::fs::read_to_string(&path).expect("could not read file");

    for line in content.lines() {
        if line.starts_with("Register A:") {
            a = line.split(": ").nth(1).expect("Error").parse().expect("Error parsing");
        } else if line.starts_with("Register B:") {
            b = line.split(": ").nth(1).expect("Error").parse().expect("Error parsing");
        } else if line.starts_with("Register C:") {
            c = line.split(": ").nth(1).expect("Error").parse().expect("Error parsing");
        } else if line.starts_with("Program:") {
            program = line.split(": ").nth(1).expect("Error").split(",").map(|n| n.parse().expect("Error parsing")).collect();
        }
    }
    return Computer {
        a,b,c,program, instruction_counter: 0
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
    program: Vec<u8>,
    instruction_counter: usize
}

fn iterate(mutable: &mut Computer, output: &mut Vec<u128>) -> bool {
    let opcode = mutable.program[mutable.instruction_counter];
    let literal = mutable.program[mutable.instruction_counter + 1];

    let combo: u128;

    if literal < 4 {
        combo = literal as u128;
    } else {
        match literal {
            4 => {
                combo = mutable.a;
            },
            5 => {
                combo = mutable.b;
            },
            6 => {
                combo = mutable.c;
            },
            _ => {
                panic!("Invalid combo operator!");
            }
        }
    }

    let mut advance_instruction_counter = true;

    match opcode {
        0 => { // adv
            let denom = 2_u128.pow(combo as u32);
            let num = mutable.a;
            mutable.a = num / denom;
        },
        1 => { // bxl
            mutable.b = mutable.b ^ literal as u128;
        },
        2 => { // bst
            mutable.b = combo % 8;
        },
        3 => { // jnz
            if mutable.a != 0 {
                advance_instruction_counter = false;
                mutable.instruction_counter = literal as usize;
            }
        },
        4 => { // bxc
            mutable.b = mutable.b ^ mutable.c;
        },
        5 => { // out
            output.push((combo % 8) as u128);
        },
        6 => { // bdv
            let denom = 2_u128.pow(combo as u32);
            let num = mutable.a;
            mutable.b = num / denom;
        },
        7 => { // cdv
            let denom = 2_u128.pow(combo as u32);
            let num = mutable.a;
            mutable.c = num / denom;
        },
        _ => {
            panic!("Unknown opcode!")
        }
    }

    if advance_instruction_counter {
        mutable.instruction_counter += 2;
        if mutable.instruction_counter > mutable.program.len() - 1 {
            return true;
        }
    }
    return false;
}

fn solve_part1(input: &Computer) -> String {
    println!("{:?}", input);

    let mut mutable = input.clone();
    let mut output: Vec<u128> = vec![];

    loop {
        println!("{:?}", mutable);
        if iterate(&mut mutable, &mut output) {
            break;
        }
    }
    
    return output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(",");
}

fn recurse(program: &Vec<u8>, a: u128) -> (bool, u128) {
    let mut our_program = program.clone();
    let result = our_program.pop().expect("Error") as u128;

    let our_a = a * 8;
    print!("{}: ", result);

    for i in 0..8 {
        let mut b = (our_a+i) % 8;
        b = b ^ 1;
        let c = (our_a+i) / (2_u128.pow(b as u32));
        b = b ^ 5;
        b = b ^ c;
        b = b % 8;

        if b == result as u128 {
            println!("found {}", our_a+i);
            if our_program.is_empty() {
                return (true, our_a+i);
            } else {
                let success = recurse(&our_program, our_a+i);
                if success.0 {
                    return success;
                }
            }
        }
    }
    return (false, 0);
}

fn solve_part2(input: &Computer) -> u128 {  
    let result = recurse(&input.program, 0);

    if !result.0 {
        panic!("unsuccessful");
    }

    let mut mutable = input.clone();
    mutable.a = result.1;
    let mut output: Vec<u128> = vec![];

    loop {
        if iterate(&mut mutable, &mut output) {
            break;
        }
    }
    
    println!("{}", output.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(","));

    return result.1;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day17_test.txt"));
        assert_eq!(solve_part1(&input), "4,6,3,5,6,3,5,2,1,0");
    }
    
}
use regex::Regex;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct XY {
    x: i128,
    y: i128
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Machine {
    button_a: XY,
    button_b: XY,
    prize: XY
}

fn match_line(line: &str) -> XY {
    let re = Regex::new(r"[a-zA-Z ]+: X[+|=]([0-9]*), Y[+|=]([0-9]*)").unwrap();

    let i: Vec<XY> = re.captures_iter(line).map(|caps| {
        let (_, [x,y]) = caps.extract();

        XY {
            x: x.parse().expect(""),
            y: y.parse().expect("")
        }
    }).collect();

    return i.first().expect("Error").clone();
}

fn parse_input(path: std::path::PathBuf) -> Vec<Machine> {
    let mut machines = Vec::new();
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    let mut lines_iterator = content.lines();

    loop {
        let button_a_line = lines_iterator.next();
        let button_b_line = lines_iterator.next();
        let prize_line = lines_iterator.next();

        if prize_line.is_none() || button_b_line.is_none() || button_a_line.is_none() {
            break;
        }
        let _ = lines_iterator.next();

        let a = match_line(button_a_line.expect(""));
        let b = match_line(button_b_line.expect(""));
        let p = match_line(prize_line.expect(""));


        machines.push(Machine {
            button_a: a,
            button_b: b,
            prize: p
        })
    }
    return machines;
}

fn solve_part1(input: &Vec<Machine>) -> u128 {
    let mut sum = 0;

    for machine in input {
        let result = dont_recurse(&machine.prize, machine);
        if result.0 {
            sum += result.1;
        }   
    }
    return sum;
}

fn dont_recurse(intermediate: &XY, machine: &Machine) -> (bool, u128) {
    let mut x1 = (intermediate.x as f64 / machine.button_b.x as f64 - intermediate.y as f64 / machine.button_b.y as f64) / (machine.button_a.x as f64 / machine.button_b.x as f64 - machine.button_a.y as f64 / machine.button_b.y as f64);
    let mut x2 = (intermediate.x as f64 - machine.button_a.x  as f64 * x1) / machine.button_b.x as f64;
    x1 = (x1 * 1000.0).round() / 1000.0;
    x2 = (x2 * 1000.0).round() / 1000.0;

    if x1.fract() == 0.0 && x2.fract() == 0.0 {
        return (true, x1 as u128 * 3 + x2 as u128);
    }
    return (false, 0);
}

fn solve_part2(input: &Vec<Machine>) -> u128 {
    let mut sum = 0;

    for machine in input {
        let result = dont_recurse(&XY {
            x: machine.prize.x + 10000000000000,
            y: machine.prize.y + 10000000000000 
        }, machine);
        if result.0 {
            sum += result.1;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day13_test.txt"));
        assert_eq!(solve_part1(&input), 480);
    }
}
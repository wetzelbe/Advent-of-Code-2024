pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

fn parse_input(path: std::path::PathBuf) -> Vec<(Vec<u128>, u128)> {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let mut result = Vec::new();

    for line in content.lines() {
        let mut split = line.split(": ").enumerate();
        let equation_result = split.next().expect("Error parsing").1.parse::<u128>().expect("Error parsing to u128");
        let second_part = split.next();
        let numbers = second_part.expect("Error parsing").1.split(" ").map(|n| n.parse().expect("Could not parse to number")).collect();
        
        result.push((numbers, equation_result));
    }

    return result;
}

fn is_solvable(equation: &Vec<u128>, solution: u128) -> bool {
    let mut equation_copy = equation.clone();

    if equation.len() == 1 {
        return equation[0] == solution;
    }
    let last_element = equation_copy.pop().expect("Error");

    if solution % last_element == 0 {
        let equation_result_mul = solution / last_element;
        if is_solvable(&equation_copy, equation_result_mul) {
            return true;
        }
    }

    if solution >= last_element {
        let equation_result_add = solution - last_element;
        if is_solvable(&equation_copy, equation_result_add) {
            return true;
        }
    }
    return false;
}

fn solve_part1(input: &Vec<(Vec<u128>, u128)>) -> u128 {
    let mut count:u128 = 0;
    for equation in input {
        if is_solvable(&equation.0, equation.1) {
            count += equation.1;
        }
    }
    return count;
}

fn is_solvable2(equation: &Vec<u128>, solution: u128) -> bool {
    let mut equation_copy = equation.clone();

    if equation.len() == 1 {
        return equation[0] == solution;
    }
    let last_element = equation_copy.pop().expect("Error");

    if solution % last_element == 0 {
        let equation_result_mul = solution / last_element;
        if is_solvable2(&equation_copy, equation_result_mul) {
            return true;
        }
    }

    if solution >= last_element {
        let equation_result_add = solution - last_element;
        if is_solvable2(&equation_copy, equation_result_add) {
            return true;
        }
    }

    let mut solution_string: String = solution.to_string();
    let last_element_string: String = last_element.to_string();

    if solution_string.ends_with(&last_element_string) {
        solution_string.truncate(solution_string.len() - last_element_string.len());
        println!("{}", solution_string);
        if is_solvable2(&equation_copy, solution_string.parse().expect("Something is very wrong")) {
            return true;
        }
    }

    return false;
}

fn solve_part2(input: &Vec<(Vec<u128>, u128)>) -> u128 {
    let mut count:u128 = 0;
    for equation in input {
        if is_solvable2(&equation.0, equation.1) {
            count += equation.1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_equation() {
        assert!(is_solvable(&vec![81, 40, 27], 3267));
    }
    
    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day7_test.txt"));
        assert_eq!(solve_part1(&input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day7_test.txt"));
        assert_eq!(solve_part2(&input), 11387);
    }
}
use std::collections::HashMap;

pub fn solve(path: std::path::PathBuf, iterations: Option<usize>) {
    let input = parse_input(path);
    println!("Input: \n{:?}", input);
    println!("The solution is {}", solve_part1(&input, iterations.expect("Please provide number of iterations as third argument")));
}


fn parse_input(path: std::path::PathBuf) -> Vec<u128> {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    return content.lines().next().expect("INput does not have at least one line").split(" ").map(|s| s.parse().expect("Could not parse to u128")).collect();
}

fn recurse(input: u128, max: usize, store: &mut HashMap<(u128, usize), usize>) -> usize {
    println!("{}, {}", input, max);

    let stored = store.get(&(input, max));

    if stored.is_some() {
        let stored_value = stored.expect("Error");

        return *stored_value;
    }

    let mut result: Vec<u128> = Vec::new();

    if input == 0 {
        result.push(1);
    } else if input.to_string().len() % 2 == 0 {
        let mut s = input.to_string();

        let s2 = s.split_off(s.len() / 2);
        result.push(s.parse().expect("Could not parse split number, part one"));
        result.push(s2.parse().expect("Could not parse split number, part two"));
    } else {
        result.push(input * 2024);
    }

    if max == 0 {
        return result.len();
    }

    let mut sum: usize = 0;

    for n in result {
        sum += recurse(n, max - 1, store);
    }

    store.insert((input, max), sum);

    return sum;
}

fn solve_part1(input: &Vec<u128>, iterations: usize) -> usize {
    let mut store: HashMap<(u128, usize), usize> = HashMap::new();

    let mut sum: usize= 0;
    for n in input {
        println!("{}", n);
        sum += recurse(*n, iterations - 1, &mut store);
    }
    return sum;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day11_test.txt"));
        assert_eq!(solve_part1(&input, 6), 22);
    }
    
    #[test]
    fn test_part1_2() {
        let input = parse_input(PathBuf::from("./resources/day11_test.txt"));
        assert_eq!(solve_part1(&input, 25), 55312);
    }
}
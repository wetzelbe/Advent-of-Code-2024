pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("Input: {:?}", input);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}


fn parse_input(path: std::path::PathBuf) -> Vec<i32> {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let mut result: Vec<i32> = Vec::new();

    let mut free_space = false;

    let mut id = 0;

    for c in content.lines().nth(0).expect("Could not read first line").to_string().chars() {
        let l = c.to_string().parse().expect("Could not parse number");
        if free_space {
            result.append(&mut vec![-1; l]);
        } else {
            result.append(&mut vec![id; l]);
            id += 1;
        }

        free_space = !free_space;
    }

    return result;
}

fn solve_part2(input: &Vec<i32>) -> u128 {
    let mut result = input.clone();

    for i in (0..(*result.iter().max().expect("Could not get max value") + 1)).rev() {
        let l = result.iter().filter(|n| **n == i).count();
        let current_start_position = result.iter().position(|n| *n == i).expect("Could not find value");
        
        result = result.iter().map(|n| if *n == i { -2 } else { *n }).collect();

        for j in 0..current_start_position {
            let mut fits = 0;
            for m in 0..l {
                if j+m < result.len() && result[j+m] == -1 {
                    fits += 1;
                }
            }

            if fits == l {
                for m in 0..l {
                    result[j+m] = i;
                }
                result = result.iter().map(|n| if *n == -2 { -1 } else { *n }).collect();

                break;
            }
        }
        result = result.iter().map(|n| if *n == -2 { i } else { *n }).collect();
    }

    let mut i: u128 = 0;
    let mut sum: u128 = 0;
    for c in result {
        if c != -1 {
            sum += i * (c as u128);
        }
        i += 1;
    }

    return sum;
}


fn solve_part1(input: &Vec<i32>) -> u128 {
    let mut result = input.clone();

    let mut done = false;
    while !done {
        let last_iteration = result.clone();

        let mut last_element = -1;
        for i in (0..result.len()).rev() {
            if result[i] != -1 {
                last_element = result[i];
                result[i] = -1;
                break;
            }
        }

        if last_element != -1 {
            for i in 0..result.len() {
                if result[i] == -1 {
                    result[i] = last_element;
                    break;
                }
            }
        }
        if last_iteration == result {
            done = true;
        }
    }

    let mut i: u128 = 0;
    let mut sum: u128 = 0;
    for c in result {
        if c != -1 {
            sum += i * (c as u128);
        }
        i += 1;
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day9_test.txt"));
        assert_eq!(solve_part1(&input), 1928);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day9_test.txt"));
        assert_eq!(solve_part2(&input), 2858);
    }
}
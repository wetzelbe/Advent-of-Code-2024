use ndarray::Array2;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    let solution_part1 = solve_part1(&input);
    println!("{}", solution_part1.0);
    println!("The solution to part 1 is {}", solution_part1.1);

    let solution_part2 = solve_part2(&input);
    println!("The solution to part 2 is {}", solution_part2);

}

fn parse_input(path: std::path::PathBuf) -> ndarray::Array2<char> {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    let h = content.lines().count();
    let mut value: String = String::from("");
    let mut w: usize = 0;

    for line in content.lines() {
        value.extend(line.chars());
        w = line.len();
    }
    return ndarray::Array::from_shape_vec((h,w),value.chars().collect()).expect("Could not put to matrix");
}

fn current_position(input: &ndarray::Array2<char>) -> (usize, usize) {
    let index = input.flatten().iter().position(|v| *v == '^' || *v == '>' || *v == '<' || *v == 'd').expect("Could not find position");
    return (index / input.dim().1, index % input.dim().1)
}

fn solve_part1(input: &ndarray::Array2<char>) -> (ndarray::Array2<char>, u32) {
    let mut out_of_bounds = false;

    let mut solution = input.clone();

    while !out_of_bounds {
        let current_position = current_position(&solution);
        let current_direction = solution[[current_position.0, current_position.1]];

        // Check obstruction
        if current_direction == '^' {
            if current_position.0 == 0 {
                solution[[current_position.0, current_position.1]] = 'X';
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0 - 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]] == '#' {
                    solution[[current_position.0, current_position.1]] = '>';
                } else {
                    solution[[current_position.0, current_position.1]] = 'X';
                    solution[[position_ahead.0, position_ahead.1]] = current_direction;
                }
            }
        } else if current_direction == '>' {
            if current_position.1 == solution.dim().1 - 1 {
                solution[[current_position.0, current_position.1]] = 'X';
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0, current_position.1 + 1);
                if solution[[position_ahead.0, position_ahead.1]] == '#' {
                    solution[[current_position.0, current_position.1]] = 'd';
                } else {
                    solution[[current_position.0, current_position.1]] = 'X';
                    solution[[position_ahead.0, position_ahead.1]] = current_direction;
                }
            }
        } else if current_direction == 'd' {
            if current_position.0 == solution.dim().0 - 1 {
                solution[[current_position.0, current_position.1]] = 'X';
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0 + 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]] == '#' {
                    solution[[current_position.0, current_position.1]] = '<';
                } else {
                    solution[[current_position.0, current_position.1]] = 'X';
                    solution[[position_ahead.0, position_ahead.1]] = current_direction;
                }
            }
        } else if current_direction == '<' {
            if current_position.1 == 0 {
                solution[[current_position.0, current_position.1]] = 'X';
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0, current_position.1 - 1);
                if solution[[position_ahead.0, position_ahead.1]] == '#' {
                    solution[[current_position.0, current_position.1]] = '^';
                } else {
                    solution[[current_position.0, current_position.1]] = 'X';
                    solution[[position_ahead.0, position_ahead.1]] = current_direction;
                }
            }
        }
    }

    let count = solution.flatten().iter().filter(|c| **c == 'X').count() as u32;

    return (solution, count);
}

fn produces_loop(current_solution: &ndarray::Array2<String>, position_ahead: (usize, usize), position: (usize, usize)) -> bool {
    let mut solution: Array2<String> = current_solution.clone();
    solution[[position_ahead.0, position_ahead.1]].push('#');

    if solution[[position_ahead.0, position_ahead.1]].contains('^') 
        || solution[[position_ahead.0, position_ahead.1]].contains('>')
        || solution[[position_ahead.0, position_ahead.1]].contains('<')
        || solution[[position_ahead.0, position_ahead.1]].contains('d') {
            return false
        }

    let direction = current_solution[[position.0, position.1]].chars().last().expect("Could not get starting point");

    let mut first = true;

    let mut current_position = position;
    let mut current_direction = direction.clone();

    loop {
        if first {
            first = false;
        } else {
            if solution[[current_position.0, current_position.1]].contains(current_direction) {
                return true;
            } else {
                solution[[current_position.0, current_position.1]].push(current_direction);
            }
        }

        // Check obstruction
        if current_direction == '^' {
            if current_position.0 == 0 {
                return false;
            } else {
                let position_ahead = (current_position.0 - 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '>';
                } else {
                    current_position = position_ahead;
                }
            }
        } else if current_direction == '>' {
            if current_position.1 == solution.dim().1 - 1 {
                return false;
            } else {
                let position_ahead = (current_position.0, current_position.1 + 1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = 'd';
                } else {
                    current_position = position_ahead;
                }
            }
        } else if current_direction == 'd' {
            if current_position.0 == solution.dim().0 - 1 {
                return false;
            } else {
                let position_ahead = (current_position.0 + 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '<';
                } else {
                    current_position = position_ahead;
                }
            }
        } else if current_direction == '<' {
            if current_position.1 == 0 {
                return false;
            } else {
                let position_ahead = (current_position.0, current_position.1 - 1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '^';
                } else {
                    current_position = position_ahead;
                }
            }
        }
    }
}

fn solve_part2(input: &ndarray::Array2<char>) -> u32 {
    let mut out_of_bounds = false;
    let start_position = current_position(&input);
    let start_direction = input[[start_position.0, start_position.1]];

    let mut solution: Array2<String> = ndarray::Array::from_shape_vec(input.dim(),input.flatten().into_iter().map(|c| String::from(c)).collect()).expect("Could not convert to array with list as elements");
    let mut obstacle_positions: Vec<(usize, usize)> = Vec::new();

    let mut current_position = start_position;
    let mut current_direction = start_direction;

    let mut first = true;
    while !out_of_bounds {
        if first {
            first = false;
        } else {
            solution[[current_position.0, current_position.1]].push(current_direction);
        }

        // Check obstruction
        if current_direction == '^' {
            if current_position.0 == 0 {
                solution[[current_position.0, current_position.1]].push('^');
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0 - 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '>';
                } else {
                    if !obstacle_positions.contains(&position_ahead) && produces_loop(&solution, position_ahead, current_position) {
                        obstacle_positions.push(position_ahead);
                    }
                    current_position = position_ahead;
                }
            }
        } else if current_direction == '>' {
            if current_position.1 == solution.dim().1 - 1 {
                solution[[current_position.0, current_position.1]].push('>');
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0, current_position.1 + 1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = 'd';
                } else {
                    if !obstacle_positions.contains(&position_ahead) && produces_loop(&solution, position_ahead, current_position) {
                        obstacle_positions.push(position_ahead);
                    }
                    current_position = position_ahead;
                }
            }
        } else if current_direction == 'd' {
            if current_position.0 == solution.dim().0 - 1 {
                solution[[current_position.0, current_position.1]].push('d');
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0 + 1, current_position.1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '<';
                } else {
                    if !obstacle_positions.contains(&position_ahead) && produces_loop(&solution, position_ahead, current_position) {
                        obstacle_positions.push(position_ahead);
                    }
                    current_position = position_ahead;
                }
            }
        } else if current_direction == '<' {
            if current_position.1 == 0 {
                solution[[current_position.0, current_position.1]].push('<');
                out_of_bounds = true;
            } else {
                let position_ahead = (current_position.0, current_position.1 - 1);
                if solution[[position_ahead.0, position_ahead.1]].contains('#') {
                    current_direction = '^';
                } else {
                    if !obstacle_positions.contains(&position_ahead) && produces_loop(&solution, position_ahead, current_position) {
                        obstacle_positions.push(position_ahead);
                    }
                    current_position = position_ahead;
                }
            }
        }
    }
    return obstacle_positions.iter().count() as u32;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day6_test.txt"));
        assert_eq!(solve_part1(&input).1, 41);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day6_test.txt"));
        assert_eq!(solve_part2(&input), 6);
    }
}
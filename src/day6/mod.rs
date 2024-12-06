pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    let solution_part1 = solve_part1(&input);
    println!("{}", solution_part1.0);
    println!("The solution to part 1 is {}", solution_part1.1);
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
                    continue;
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
                    continue;
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day6_test.txt"));
        assert_eq!(solve_part1(&input).1, 18);
    }
}
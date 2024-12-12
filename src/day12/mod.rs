pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    //println!("Input: \n{:#}", input);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}


fn parse_input(path: std::path::PathBuf) -> ndarray::Array2<char> {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    let h = content.lines().count();
    let mut value: Vec<char> = Vec::new();
    let mut w: usize = 0;

    for line in content.lines() {
        value.append(&mut line.chars().collect());
        w = line.len();
    }
    return ndarray::Array::from_shape_vec((h,w),value).expect("Could not put to matrix");
}


fn walk(input: &mut ndarray::Array2<char>, position: (usize, usize)) -> (usize, usize) {

    let mut perimeter: usize = 0;
    let mut area: usize = 1;
    let us = input[[position.0, position.1]];

    input[[position.0, position.1]] = '1';

    if (position.0 > 0 && input[[position.0 - 1, position.1]] != us && input[[position.0 - 1, position.1]] != '1') || position.0 == 0 {
        perimeter += 1;
    }

    if (position.1 > 0 && input[[position.0, position.1 - 1]] != us && input[[position.0, position.1 - 1]] != '1') || position.1 == 0 {
        perimeter += 1;
    }
    
    if (position.0 < input.dim().0 - 1 && input[[position.0 + 1, position.1]] != us && input[[position.0 + 1, position.1]] != '1') || position.0 == input.dim().0 - 1  {
        perimeter += 1;
    }

    if (position.1 < input.dim().1 - 1 && input[[position.0, position.1 + 1]] != us && input[[position.0, position.1 + 1]] != '1') || position.1 == input.dim().1 - 1 {
        perimeter += 1;
    }

    if position.0 > 0 && input[[position.0 - 1, position.1]] == us {
        let added = walk(input, (position.0 - 1, position.1));
        perimeter += added.0;
        area += added.1;
    } 
    
    if position.1 > 0 && input[[position.0, position.1 - 1]] == us {
        let added = walk(input, (position.0, position.1 - 1));
        perimeter += added.0;
        area += added.1;
    } 
    
    if position.0 < input.dim().0 - 1 && input[[position.0 + 1, position.1]] == us {
        let added = walk(input, (position.0 + 1, position.1));
        perimeter += added.0;
        area += added.1;
    } 
    
    if position.1 < input.dim().1 - 1 && input[[position.0, position.1 + 1]] == us {
        let added = walk(input, (position.0, position.1 + 1));
        perimeter += added.0;
        area += added.1;
    }

    input[[position.0, position.1]] = '0';

    return (perimeter, area);
}


fn solve_part1(input: &ndarray::Array2<char>) -> usize {
    let mut sum: usize = 0;

    let mut workable_input = input.clone();

    while !workable_input.flatten().iter().all(|c| *c == '0') {
        let mut end: bool = false;
        for i in 0..workable_input.dim().0 {
            for j in 0..workable_input.dim().1 {
                if workable_input[[i, j]] != '0' {
                    
                    let region_result = walk(&mut workable_input, (i,j));
                    sum += region_result.0 * region_result.1;

                    end = true;
                    break;
                }
            }
            if end {
                break;
            }
        }
    }

    return sum;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

fn walk2(input: &mut ndarray::Array2<char>, position: (usize, usize)) -> (Vec<((usize, usize), Direction)>, usize) {
    let mut area: usize = 1;
    let us = input[[position.0, position.1]];

    let mut edges: Vec<((usize, usize), Direction)> = Vec::new();

    input[[position.0, position.1]] = '1';

    if (position.0 > 0 && input[[position.0 - 1, position.1]] != us && input[[position.0 - 1, position.1]] != '1') || position.0 == 0 {
        edges.push((position, Direction::Top));
    }

    if (position.1 > 0 && input[[position.0, position.1 - 1]] != us && input[[position.0, position.1 - 1]] != '1') || position.1 == 0 {
        edges.push((position, Direction::Left));
    }
    
    if (position.0 < input.dim().0 - 1 && input[[position.0 + 1, position.1]] != us && input[[position.0 + 1, position.1]] != '1') || position.0 == input.dim().0 - 1  {
        edges.push((position, Direction::Bottom));
    }

    if (position.1 < input.dim().1 - 1 && input[[position.0, position.1 + 1]] != us && input[[position.0, position.1 + 1]] != '1') || position.1 == input.dim().1 - 1 {
        edges.push((position, Direction::Right));
    }

    if position.0 > 0 && input[[position.0 - 1, position.1]] == us {
        let mut added = walk2(input, (position.0 - 1, position.1));
        edges.append(&mut added.0);
        area += added.1;
    } 
    
    if position.1 > 0 && input[[position.0, position.1 - 1]] == us {
        let mut added = walk2(input, (position.0, position.1 - 1));
        edges.append(&mut added.0);
        area += added.1;
    } 
    
    if position.0 < input.dim().0 - 1 && input[[position.0 + 1, position.1]] == us {
        let mut added = walk2(input, (position.0 + 1, position.1));
        edges.append(&mut added.0);
        area += added.1;
    } 
    
    if position.1 < input.dim().1 - 1 && input[[position.0, position.1 + 1]] == us {
        let mut added = walk2(input, (position.0, position.1 + 1));
        edges.append(&mut added.0);
        area += added.1;
    }

    return (edges, area);
}

fn merge_and_count(input: &mut Vec<((usize, usize), Direction)>) -> usize {
    let mut merged:Vec<Vec<((usize, usize), Direction)>> = Vec::new();

    while input.len() > 0 {
        let mut none_fit = true;
        let mut fit: usize = 0;
        for (i, test) in input.iter().enumerate() {
            let mut fits = false;
            let mut group_id: usize = 0;
            for (j, group) in merged.iter().enumerate() {
                for edge in group {
                    if edge.1 == test.1 {
                        match edge.1 {
                            Direction::Bottom => {
                                if edge.0.0 == test.0.0 && test.0.1.abs_diff(edge.0.1) == 1 {
                                    fits = true;
                                }
                            },
                            Direction::Top => {
                                if edge.0.0 == test.0.0 && test.0.1.abs_diff(edge.0.1) == 1 {
                                    fits = true;
                                }
                            },
                            Direction::Left => {
                                if edge.0.1 == test.0.1 && test.0.0.abs_diff(edge.0.0) == 1 {
                                    fits = true;
                                }
                            },
                            Direction::Right => {
                                if edge.0.1 == test.0.1 && test.0.0.abs_diff(edge.0.0) == 1 {
                                    fits = true;
                                }
                            }
                        }
                        if fits {
                            group_id = j;
                            break;
                        }
                    }
                }
                if fits {
                    merged[group_id].push(*test);
                    break;
                }
            }
            if fits {
                fit = i;
                none_fit = false;
                break;
            }
        }
        if none_fit {
            merged.push(vec![input.pop().expect("Somehow there was an error")]);
        } else {
            input.remove(fit);
        }
    }
    return merged.len();
}


fn solve_part2(input: &ndarray::Array2<char>) -> usize {
    let mut sum: usize = 0;

    let mut workable_input = input.clone();

    while !workable_input.flatten().iter().all(|c| *c == '0') {
        let mut end: bool = false;
        for i in 0..workable_input.dim().0 {
            for j in 0..workable_input.dim().1 {
                if workable_input[[i, j]] != '0' {
                    let mut region_result = walk2(&mut workable_input, (i,j));
                    
                    let value = workable_input.flatten().iter().map(|c| if *c == '1' { '0' } else { *c }).collect();
                    workable_input = ndarray::Array::from_shape_vec(workable_input.dim(),value).expect("Could not put to matrix");
                    let sides = merge_and_count(&mut region_result.0);
                    sum += sides * region_result.1;
                    end = true;
                    break;
                }
            }
            if end {
                break;
            }
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
        let input = parse_input(PathBuf::from("./resources/day12_test.txt"));
        assert_eq!(solve_part1(&input), 1930);
    }

    
    #[test]
    fn test_part1_2() {
        let input = parse_input(PathBuf::from("./resources/day12_test_2.txt"));
        assert_eq!(solve_part1(&input), 772);
    }
    
    #[test]
    fn test_part1_3() {
        let input = parse_input(PathBuf::from("./resources/day12_test_3.txt"));
        assert_eq!(solve_part1(&input), 140);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day12_test.txt"));
        assert_eq!(solve_part2(&input), 1206);
    }

    
    #[test]
    fn test_part2_2() {
        let input = parse_input(PathBuf::from("./resources/day12_test_2.txt"));
        assert_eq!(solve_part2(&input), 436);
    }
    
    #[test]
    fn test_part2_3() {
        let input = parse_input(PathBuf::from("./resources/day12_test_3.txt"));
        assert_eq!(solve_part2(&input), 80);
    }
    
    #[test]
    fn test_part2_4() {
        let input = parse_input(PathBuf::from("./resources/day12_test_4.txt"));
        assert_eq!(solve_part2(&input), 236);
    }
}
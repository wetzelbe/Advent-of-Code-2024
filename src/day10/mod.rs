pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("Input: \n{:#}", input);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}


fn parse_input(path: std::path::PathBuf) -> ndarray::Array2<u32> {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    let h = content.lines().count();
    let mut value: Vec<u32> = Vec::new();
    let mut w: usize = 0;

    for line in content.lines() {
        value.append(&mut line.chars().map(|c| c.to_string().parse::<u32>().expect("Could not parse number")).collect());
        w = line.len();
    }
    return ndarray::Array::from_shape_vec((h,w),value).expect("Could not put to matrix");
}

fn solve_part2(input: &ndarray::Array2<u32>) -> usize {
    let trailheads = find_all_coordinates_with_value(input, 0);

    let mut sum: usize = 0;

    for trailhead in trailheads {
        sum += recurse(input, 0, trailhead, false).iter().count();
    }

    return sum;
}


fn find_all_coordinates_with_value(input: &ndarray::Array2<u32>, value: u32) -> Vec<(usize, usize)> {
    let mut coordinates: Vec<(usize, usize)> = Vec::new();

    for i in 0..input.dim().0 {
        for j in 0..input.dim().1 {
            if input[[i,j]] == value {
                coordinates.push((i, j));
            }
        }
    }

    return coordinates;
} 

fn find_neighbours_with_value(input: &ndarray::Array2<u32>, value: u32, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    if position.0 > 0 && input[[position.0 - 1, position.1]] == value {
        neighbours.push((position.0 - 1, position.1));
    }

    if position.1 > 0 && input[[position.0, position.1 - 1]] == value {
        neighbours.push((position.0, position.1 - 1));
    }

    if position.0 < input.dim().0 - 1 && input[[position.0 + 1, position.1]] == value {
        neighbours.push((position.0 + 1, position.1));
    }

    if position.1 < input.dim().1 - 1 && input[[position.0, position.1 + 1]] == value {
        neighbours.push((position.0, position.1 + 1));
    }

    return neighbours;
}


fn recurse(input: &ndarray::Array2<u32>, value: u32, position: (usize, usize), filter: bool) -> Vec<(usize, usize)> {
    let mut reachable_ends: Vec<(usize, usize)> = Vec::new();

    // Stop condition
    if value == 9 && input[[position.0, position.1]] == 9 {
        reachable_ends.push(position);
        return reachable_ends;
    }

    let neighbours = find_neighbours_with_value(input, value + 1, position);

    for neighbour in neighbours {
        let mut recursion_result = recurse(input, value + 1, neighbour, filter);
        if filter {
            for result in recursion_result {
                if reachable_ends.iter().all(|end| *end != result) {
                    reachable_ends.push(result);
                }
            }
        } else {
            reachable_ends.append(&mut recursion_result);
        }
    }

    return reachable_ends;
}

fn solve_part1(input: &ndarray::Array2<u32>) -> usize {
    let trailheads = find_all_coordinates_with_value(input, 0);

    let mut sum: usize = 0;

    for trailhead in trailheads {
        sum += recurse(input, 0, trailhead, true).iter().count();
    }

    return sum;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day10_test.txt"));
        assert_eq!(solve_part1(&input), 36);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day10_test.txt"));
        assert_eq!(solve_part2(&input), 81);
    }
}
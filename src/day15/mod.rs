use ndarray::Array2;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("\nThe solution to part 1 is {}\n", solve_part1(&input.0, &input.1));
    println!("The solution to part 2 is {}", solve_part2(&input.0, &input.1));
}


fn parse_input(path: std::path::PathBuf) -> (Array2<char>, Vec<char>) {
    let mut field: Vec<char> = Vec::new();
    let content = std::fs::read_to_string(&path).expect("could not read file");
    let mut w = 0;
    let mut h = 0;

    let mut phase2 = false;
    let mut moves: Vec<char> = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            phase2 = true;
        } else if !phase2 {
            h += 1;
            field.append(&mut line.chars().collect());
            w = line.len();
        } else {
            moves.append(&mut line.chars().collect());
        }
    }

    return (ndarray::Array::from_shape_vec((h,w),field).expect("Could not put to matrix"), moves);
}

fn find_position(input: &Array2<char>) -> (usize, usize) {
    for y in 0..input.dim().1 {
        for x in 0..input.dim().0 {
            if input[[x,y]] == '@' {
                return (x,y);
            }
        }
    }
    return (0,0);
}

fn recurse(input: &mut Array2<char>, position: (usize, usize), vector: (i32, i32)) -> bool {
    let new_position = ((position.0 as i32 + vector.0) as usize, (position.1 as i32 + vector.1) as usize);
    if input[[new_position.0, new_position.1]] == '#' {
        return false;
    }
    if input[[new_position.0, new_position.1]] == '.' {
        input[[new_position.0, new_position.1]] = input[[position.0, position.1]];
        input[[position.0, position.1]] = 'x';
        return true;
    }
    if input[[new_position.0, new_position.1]] == 'O' || input[[new_position.0, new_position.1]] == '[' || input[[new_position.0, new_position.1]] == ']' {
        if recurse(input, new_position, vector) {
            input[[new_position.0, new_position.1]] = input[[position.0, position.1]];
            input[[position.0, position.1]] = 'x';
            return true;
        }
    }
    return false;
}

fn iterate(input: &Array2<char>, direction: char) -> Array2<char> {
    let mut editable = input.clone();

    let mut vector: (i32, i32) = (0,0);

    if direction == '^' {
        vector = (-1,0);
    } else if direction == '>' {
        vector = (0,1);
    } else if direction == 'v' {
        vector = (1,0);
    } else if direction == '<' {
        vector = (0,-1);
    }

    let position = find_position(input);
    let new_position = ((position.0 as i32 + vector.0) as usize, (position.1 as i32 + vector.1) as usize);
    if recurse(&mut editable, position, vector) {
        editable[[new_position.0, new_position.1]] = '@';
        editable[[position.0, position.1]] = '.';
    }

    return editable;
}

fn count(input: &Array2<char>) -> usize {
    let mut count: usize = 0;

    for i in 0..input.dim().0 {
        for j in 0..input.dim().1 {
            if input[[i,j]] == 'O' || input[[i,j]] == '[' {
                count += i * 100;
                count += j;
            }
        }
    }

    return count;
}


fn solve_part1(input: &Array2<char>, moves: &Vec<char>) -> usize {

    let mut editable = input.clone();
    for m in moves {
        editable = iterate(&editable, *m);
    }
    println!("{:#}", editable);

    return count(&editable);
}

fn resize(input: &Array2<char>) -> Array2<char> {
    let dim = input.dim();

    let mut result: Vec<char> = Vec::new();

    for c in input.flatten() {
        if c == '#' {
            result.push('#');
            result.push('#');
        } else if c == 'O' {
            result.push('[');
            result.push(']');
        } else if c == '.' {
            result.push('.');
            result.push('.');
        } else if c == '@' {
            result.push('@');
            result.push('.');
        }
    }
    return ndarray::Array::from_shape_vec((dim.0, 2 * dim.1),result).expect("Could not put to matrix");
}   


fn vector_for_direction(direction: char) -> (i32, i32) {
    if direction == '^' {
        return (-1,0);
    } else if direction == '>' {
        return (0,1);
    } else if direction == 'v' {
        return (1,0);
    } else if direction == '<' {
        return (0,-1);
    }
    return (0,0);
}

fn can_move_v(input: &mut Array2<char>, position: (usize, usize), direction: char) -> bool {
    let vector = vector_for_direction(direction);
    let new_position = ((position.0 as i32 + vector.0) as usize, (position.1 as i32 + vector.1) as usize);

    if input[[new_position.0, new_position.1]] == '#' {
        return false;
    }
    if input[[new_position.0, new_position.1]] == '.' {
        return true;
    }
    if input[[new_position.0, new_position.1]] == '[' {
        let second_new_position = (new_position.0, new_position.1 + 1);
        return can_move_v(input, new_position, direction) && can_move_v(input, second_new_position, direction);
    }
    if input[[new_position.0, new_position.1]] == ']' {
        let second_new_position = (new_position.0, new_position.1 - 1);
        return can_move_v(input, new_position, direction) && can_move_v(input, second_new_position, direction);
    }
    return false;
}

fn do_move(input: &mut Array2<char>, position: (usize, usize), direction: char) {
    let vector = vector_for_direction(direction);
    let new_position = ((position.0 as i32 + vector.0) as usize, (position.1 as i32 + vector.1) as usize);

    if input[[new_position.0, new_position.1]] == '.' {
        input[[new_position.0, new_position.1]] = input[[position.0, position.1]];
        input[[position.0, position.1]] = '.';
    }
    else if input[[new_position.0, new_position.1]] == '[' {
        let second_new_position = (new_position.0, new_position.1 + 1);
        do_move(input, new_position, direction);
        do_move(input, second_new_position, direction);
        input[[new_position.0, new_position.1]] = input[[position.0, position.1]];
        input[[position.0, position.1]] = '.';
    }
    else if input[[new_position.0, new_position.1]] == ']' {
        let second_new_position = (new_position.0, new_position.1 - 1);
        do_move(input, new_position, direction);
        do_move(input, second_new_position, direction);
        input[[new_position.0, new_position.1]] = input[[position.0, position.1]];
        input[[position.0, position.1]] = '.';
    }
}

fn iterate2(input: &Array2<char>, direction: char) -> Array2<char> {
    let mut editable = input.clone();

    let vector = vector_for_direction(direction);

    let position = find_position(input);

    let new_position = ((position.0 as i32 + vector.0) as usize, (position.1 as i32 + vector.1) as usize);

    if direction == '^' || direction == 'v' {
        if can_move_v(&mut editable, position, direction) {
            do_move(&mut editable, position, direction);
        }
    } else {
        if recurse(&mut editable, position, vector) {
            editable[[new_position.0, new_position.1]] = '@';
            editable[[position.0, position.1]] = '.';
        }
    }

    return editable;
}


fn solve_part2(input: &Array2<char>, moves: &Vec<char>) -> usize {
    
    let mut editable = resize(input);
    for m in moves {
        editable = iterate2(&editable, *m);
    }
    println!("{:#}", editable);
    return count(&editable);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day15_test.txt"));
        assert_eq!(solve_part1(&input.0, &input.1), 10092);
    }
    
    #[test]
    fn test_part1_2() {
        let input = parse_input(PathBuf::from("./resources/day15_test_2.txt"));
        assert_eq!(solve_part1(&input.0, &input.1), 2028);
    }
    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day15_test.txt"));
        assert_eq!(solve_part2(&input.0, &input.1), 9021);
    }
}
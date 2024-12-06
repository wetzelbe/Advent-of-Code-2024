use ndarray;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);

    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

fn parse_input(path: std::path::PathBuf) -> ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>> {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let h = content.lines().count();
    let mut w: usize = 0;

    let mut value: String = String::from("");
    for line in content.lines() {
        value.extend(line.chars());
        w = line.len();
    }

    return ndarray::Array::from_shape_vec((h,w),value.chars().collect()).expect("Could not put to matrix");
}

fn solve_part1(input: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>) -> u32 {
    let mut count: u32 = 0;
    // horizontal
    for row in input.rows() {
        let s: String = row.to_vec().iter().collect();
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }

    // vertical
    for col in input.columns() {
        let s: String = col.to_vec().iter().collect();
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }

    // top left to bottom right and reverse
    for j in 0..input.dim().1 {
        let mut s = String::from("");
        let longest_diagonal_length = std::cmp::min(input.dim().0, input.dim().1 - j);
        for i in 0..longest_diagonal_length {
            s.push(input[[i, i + j]]);
        }
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }
    for j in 1..input.dim().0 {
        let mut s = String::from("");
        let longest_diagonal_length = std::cmp::min(input.dim().0 - j, input.dim().1);
        for i in 0..longest_diagonal_length {
            s.push(input[[i + j, i]]);
        }
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }

    // bottom left to top right
    for j in 0..input.dim().1 {
        let mut s = String::from("");
        let longest_diagonal_length = std::cmp::min(input.dim().0, input.dim().1 - j);
        for i in 0..longest_diagonal_length {
            s.push(input[[input.dim().0 - 1 - i, i + j]]);
        }
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }
    for j in 1..input.dim().1 {
        let mut s = String::from("");
        let longest_diagonal_length = std::cmp::min(input.dim().0 - j, input.dim().1);
        for i in 0..longest_diagonal_length {
            s.push(input[[input.dim().0 - 1 - i - j, i]]);
        }
        count += s.matches("XMAS").count() as u32;
        count += s.chars().rev().collect::<String>().matches("XMAS").count() as u32;
    }
    return count;
}

fn solve_part2(input: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>) -> u32 {
    let mut count: u32 = 0;
    let dim = input.dim();
    for i in 1..(dim.0-1) {
        for j in 1..(dim.1-1) {
            if check(&input, i,j) {
                count += 1;
            }
        }
    }

    return count;
}

fn check(input: &ndarray::ArrayBase<ndarray::OwnedRepr<char>, ndarray::Dim<[usize; 2]>>, x: usize, y: usize) -> bool {
    return input[[x,y]] == 'A' 
        && ((input[[x-1, y-1]] == 'M' && input[[x+1,y+1]] == 'S') || (input[[x-1, y-1]] == 'S' && input[[x+1,y+1]] == 'M')) 
        && ((input[[x-1, y+1]] == 'M' && input[[x+1,y-1]] == 'S') || (input[[x-1, y+1]] == 'S' && input[[x+1,y-1]] == 'M')) ;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day4_test.txt"));
        assert_eq!(solve_part1(&input), 18);
    }
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day4_test.txt"));
        assert_eq!(solve_part2(&input), 9);
    }
    
}
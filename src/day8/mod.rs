use ndarray::Array2;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("{:#}", input);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

fn parse_input(path: std::path::PathBuf) -> Array2<char> {
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

fn solve_part1(input: &Array2<char>) -> u32 {
    let mut result = input.clone();
    result.fill('.');

    for i in 0..input.dim().0 {
        for j in 0..input.dim().1 {
            if input[[i,j]] != '.' {
                for m in i..input.dim().0 {
                    for n in 0..input.dim().1 {
                        if (i != m || j != n) && input[[i,j]] == input[[m,n]] {
                            if 2 * m >= i && 2 * n >= j && 2 * m - i < result.dim().0 && 2 * n - j < result.dim().1 {
                                result[[2 * m - i, 2 * n - j]] = '#';
                            }
                            if 2 * i >= m && 2 * j >= n && 2 * i - m < result.dim().0 && 2 * j - n < result.dim().1 {
                                result[[2 * i - m, 2 * j - n]] = '#';
                            }
                        }
                    }
                }
            }
        }
    }
    return result.flatten().iter().filter(|c| **c == '#').count() as u32;
}

fn solve_part2(input: &Array2<char>) -> u32 {
    let mut result = input.clone();
    result.fill('.');

    for i in 0..input.dim().0 {
        for j in 0..input.dim().1 {
            if input[[i,j]] != '.' {
                for m in i..input.dim().0 {
                    for n in 0..input.dim().1 {
                        if (i != m || j != n) && input[[i,j]] == input[[m,n]] {
                            result[[i,j]] = '#';
                            result[[m,n]] = '#';

                            let mut x: usize = 1;
                            while (1 + x) * m >= x * i && (1 + x) * n >= x * j && (1 + x) * m - x * i < result.dim().0 && (1 + x) * n - x * j < result.dim().1 {
                                result[[(1 + x) * m - x * i, (1 + x) * n - x * j]] = '#';
                                x += 1;
                            }

                            x = 1;
                            while (1 + x) * i >= x * m && (1 + x) * j >= x * n && (1 + x) * i - x * m < result.dim().0 && (1 + x) * j - x * n < result.dim().1 {
                                result[[(1 + x) * i - x * m, (1 + x) * j - x * n]] = '#';
                                x += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:#}", result);

    return result.flatten().iter().filter(|c| **c == '#').count() as u32;
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day8_test.txt"));
        assert_eq!(solve_part1(&input), 14);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day8_test.txt"));
        assert_eq!(solve_part2(&input), 34);
    }
}
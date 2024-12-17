use ndarray::Array2;
use regex::Regex;

pub fn solve(path: std::path::PathBuf, x: Option<usize>, y: Option<usize>) {
    let input = Field {
        robots: parse_input(path),
        size: XY {
            x: x.expect("Specify field size as command line args") as i128, 
            y: y.expect("Specify field size as command line args") as i128
        }
    };
    println!("The solution to part 1 is {}", solve_part1(&input));
    solve_part2(&input);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct XY {
    x: i128,
    y: i128
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Robot {
    position: XY,
    vector: XY
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Field {
    robots: Vec<Robot>,
    size: XY
}

fn match_line(line: &str) -> Robot {
    let re = Regex::new(r"p=([0-9]+),([0-9]+) v=([-]?[0-9]+),([-]?[0-9]+)").unwrap();
    println!("{}", line);
    let i: Vec<Robot> = re.captures_iter(line).map(|caps| {
        let (_, [x,y,vx,vy]) = caps.extract();
        Robot {
            position: XY {
                x: x.parse().expect(""),
                y: y.parse().expect("")
            },
            vector: XY {
                x: vx.parse().expect(""),
                y: vy.parse().expect("")
            }
        }
    }).collect();

    return i.first().expect("Error").clone();
}

fn parse_input(path: std::path::PathBuf) -> Vec<Robot> {
    let mut robots = Vec::new();
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    for line in content.lines() {
        robots.push(match_line(line));
    }
    return robots;
}

fn iterate(field: &Field) -> Field {
    let mut robots = Vec::new();

    for robot in &field.robots {

        let mut x  = (robot.position.x + robot.vector.x) % field.size.x;
        let mut y  = (robot.position.y + robot.vector.y) % field.size.y;

        if x < 0 {
            x += field.size.x;
        }

        if y < 0 {
            y += field.size.y;
        }

        robots.push(
            Robot { 
                position: XY {
                    x,
                    y
                }, 
                vector: robot.vector.clone() }
        );
    }
    return Field {
        robots,
        size: field.size.clone()
    }
}

fn count(field: &Field) -> usize {
    let mut quadrants: (usize, usize, usize, usize) = (0,0,0,0);

    let mid_x = field.size.x / 2;
    let mid_y = field.size.y / 2;

    for robot in &field.robots {
        if robot.position.x < mid_x && robot.position.y < mid_y {
            quadrants.0 += 1;
        } else if robot.position.x > mid_x && robot.position.y < mid_y {
            quadrants.1 += 1;
        } else if robot.position.x < mid_x && robot.position.y > mid_y {
            quadrants.2 += 1;
        } else if robot.position.x > mid_x && robot.position.y > mid_y {
            quadrants.3 += 1;
        }
    }

    return quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;
}

fn solve_part1(input: &Field) -> usize {

    let mut field = input.clone();

    for _ in 0..100 {
        field = iterate(&field);
    }

    return count(&field);
}

fn display(input: &Field) -> bool {
    let mut array: Array2<char> = Array2::from_elem((input.size.x as usize, input.size.y as usize), '.');

    for robot in &input.robots {
        array[[robot.position.x as usize, robot.position.y as usize]] = 'X';
    }

    let array_as_string: String = array.flatten().to_vec().iter().collect();

    
    if array_as_string.contains("XXXXXXXXXXXXXXX") {
        for y in 0..array.dim().1 {
            for x in 0..array.dim().0 {
                print!("{}", array[[x,y]])
            }
            println!("");
        }
        return true;
    }
    return false;
}

fn solve_part2(input: &Field) {
    let mut field = input.clone();
    let mut i = 1;
    loop {
        field = iterate(&field);
        if display(&field) {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day14_test.txt"));

        let field = Field {
            robots: input,
            size: XY {
                x: 11,
                y: 7
            }
        };

        assert_eq!(solve_part1(&field), 12);
    }
}
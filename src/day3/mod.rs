use regex::Regex;

pub fn solve(path: std::path::PathBuf) {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    println!("The solution is {}", solve_string(&content));
    println!("The solution for part 2 is {}", solve_string_part2(&content));
}

fn solve_string(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut value: u32 = 0;
    let _: Vec<(&str, &str)>  = re.captures_iter(input).map(|caps| {
        let (_, [a_str, b_str]) = caps.extract();
        
        let a: u32 = a_str.parse().expect("Error");
        let b: u32 = b_str.parse().expect("Error");

        value += a*b;
        (a_str, b_str)
    }).collect();

    return value;
}

fn solve_string_part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut value: u32 = 0;
    let mut enabled: bool = true;
    let _: Vec<(&str, &str)>  = re.find_iter(input).map(|found| {
        if found.as_str() == "do()" {
            enabled = true;
        } else if found.as_str() == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let _: Vec<(&str, &str)> = mul_re.captures_iter(found.as_str()).map(|caps| {
                    let (_, [a_str, b_str]) = caps.extract();
                    
                    let a: u32 = a_str.parse().expect("Error");
                    let b: u32 = b_str.parse().expect("Error");
            
                    value += a*b;
                    (a_str, b_str)
                }).collect();
            }
        }

        ("","")
    }).collect();

    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve_string(input), 161);
    }
    
    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_string_part2(input), 48);
    }
}
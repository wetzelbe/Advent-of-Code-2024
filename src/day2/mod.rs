pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    let safety_score_value = safety_score(&input, is_safe);
    let dampened_safety_score_value = safety_score(&input, dampened_is_safe);
    println!("The safety score is {:?}", safety_score_value);
    println!("The dampened safety score is {:?}", dampened_safety_score_value);
}

fn parse_input(path: std::path::PathBuf) -> Vec<Vec<u32>> {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let mut matrix: Vec<Vec<u32>> = Vec::new();

    
    for line in content.lines() {
        let split_line = line.split(" ").enumerate();

        let mut report: Vec<u32> = Vec::new();

        for value_string in split_line {
            report.push(value_string.1.parse().expect("Could not parse string to number"));
        }

        matrix.push(report);
    }

    return matrix;
}

fn safety_score(input: &Vec<Vec<u32>>, safety_function: fn(&Vec<u32>) -> bool) -> u16 {
    let mut score: u16 = 0;

    for report in input {
        if safety_function(report) {
            score += 1;
        }
    }

    return score;
}

fn is_safe(report: &Vec<u32>) -> bool {
    let mut last: u32 = 0;

    let mut is_increasing: bool = true;
    let mut initialized: bool = false;

    for x in report {
        if last != 0 {
            if x.abs_diff(last) < 1 || x.abs_diff(last) > 3 {
                return false;
            } else {
                if !initialized {
                    is_increasing = *x > last;
                    initialized = true;
                } else {
                    if (is_increasing && *x < last) || (!is_increasing && *x > last) {
                        return false;
                    }
                }
            }
        }
        last = *x;
    }

    return true;
}

fn dampened_is_safe(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let mut report_copy = report.to_vec();
        report_copy.remove(i);
        if is_safe(&report_copy) {
            return true;
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_safety_score() {
        let path = PathBuf::from("./resources/day2_test.txt");
        let input = parse_input(path);
        assert_eq!(safety_score(&input, is_safe), 2);
    }

    #[test]
    fn test_dampened_safety_score() {
        let path = PathBuf::from("./resources/day2_test.txt");
        let input = parse_input(path);
        assert_eq!(safety_score(&input, dampened_is_safe), 4);
    }
}
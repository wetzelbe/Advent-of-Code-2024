pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("The solution of part 1 is {}", solve_part1(&input.0, &input.1));
    println!("The solution of part 2 is {}", solve_part2(&input.0, &input.1));
}

fn parse_input(path: std::path::PathBuf) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut second_part = false;

    for line in content.lines() {
        if line.is_empty() {
            second_part = true;
            continue;
        }

        if !second_part {
            let tuple: Vec<u32> = line.split("|").map(|f| f.parse().expect("could not parse")).collect();
            rules.push((tuple[0], tuple[1]));
        } else {
            let update: Vec<u32> = line.split(",").map(|f| f.parse().expect("could not parse")).collect();
            updates.push(update);
        }
    }

    return (rules, updates);
}

fn test(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    for rule in rules {
        if update.contains(&rule.0) && update.contains(&rule.1) {
            let a = update.iter().position(|&e| e == rule.0).unwrap();
            let b = update.iter().position(|&e| e == rule.1).unwrap();
            if b < a {
                return false;
            }
        }
    }

    return true;
}

fn solve_part1(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for update in updates {
        if test(&rules, &update) {
            sum += &update[update.len() / 2];
        }
    }

    return sum;
}

fn sort_rules(rules: &Vec<&(u32, u32)>) -> Vec<u32> {
    let mut remaining_rules = rules.clone();

    let remaining_right: Vec<u32> = remaining_rules.iter().map(|x| x.1).collect();
    let remaining_left: Vec<u32> = remaining_rules.iter().map(|x| x.0).collect();

    let largest_number = remaining_right.iter().filter(|a| !remaining_left.contains(a)).collect::<Vec<&u32>>()[0];

    let mut sorted: Vec<u32> = Vec::new();

    while remaining_rules.len() > 0 {
        let remaining_right: Vec<u32> = remaining_rules.iter().map(|x| x.1).collect();
        let remaining_left: Vec<u32> = remaining_rules.iter().map(|x| x.0).collect();
        let smallest_number = remaining_left.iter().filter(|a| !remaining_right.contains(a)).collect::<Vec<&u32>>()[0];
        sorted.push(*smallest_number);
        remaining_rules = remaining_rules.iter().filter(|rule| rule.0 != *smallest_number).map(|u| *u).collect();
    }

    sorted.push(*largest_number);

    return sorted;
}

fn sort(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> Vec<u32> {
    let filtered_rules: Vec<&(u32, u32)> = rules.iter().filter(|rule| update.contains(&rule.0) && update.contains(&rule.1)).collect();

    let sorted_rules = sort_rules(&filtered_rules);
    let mut update_clone = update.clone();
    update_clone.sort_by(|a,b| sorted_rules.iter().position(|e| e == a).expect(&format!("did not find number in rules {}", a)).cmp(&sorted_rules.iter().position(|e| e == b).expect(&format!("did not find number in rules {}", b))));
    return update_clone
}

fn solve_part2(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    let incorrectly_ordered_updates: Vec<&Vec<u32>> = updates.iter().filter(|update| !test(&rules, update)).collect();
    let mut sum: u32 = 0;

    for update in incorrectly_ordered_updates {
        let sorted = sort(&rules, update);
        sum += &sorted[sorted.len() / 2];
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day5_test.txt"));
        assert_eq!(solve_part1(&input.0, &input.1), 143);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day5_test.txt"));
        assert_eq!(solve_part2(&input.0, &input.1), 123);
    }

    #[test]
    fn test_sort() {
        let input = parse_input(PathBuf::from("./resources/day5_test.txt"));
        assert!(test(&input.0, &sort_rules(&input.0.iter().map(|rule| rule).collect())))
    }
}
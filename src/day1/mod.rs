pub fn solve(path: std::path::PathBuf) {

    let sets = parse_input(path);
    let similarity = calculate_similarity(&sets.0, &sets.1);
    let sum = calculate_sum_of_distances(sets.0, sets.1);
    println!(" The sum is {:?}", sum);
    println!(" The similarity is {:?}", similarity);
}

fn parse_input(path: std::path::PathBuf) -> (Vec<u32>, Vec<u32>) {
    let content = std::fs::read_to_string(&path).expect("could not read file");

    let mut set1: Vec<u32> = Vec::new();
    let mut set2: Vec<u32> = Vec::new();

    for line in content.lines() {
        let mut split_line = line.split("   ").enumerate();

        let first_value_string = split_line.next().expect("Could not get value from file").1;
        let second_value_string = split_line.next().expect("Could not get value from file").1;

        let first_value: u32 = first_value_string.parse().expect("Could not parse value in file to number, this probably is not the correct file");
        let second_value: u32 = second_value_string.parse().expect("Could not parse value in file to number, this probably is not the correct file");

        set1.push(first_value);
        set2.push(second_value);
    }

    return (set1, set2);
}

fn calculate_sum_of_distances(mut set1: Vec<u32>, mut set2: Vec<u32>) -> u32 {
    // sort our vectors
    set1.sort();
    set2.sort();

    let zipped = set1.iter().zip(set2.iter());
    
    let mut sum: u32 = 0;

    for (a, b) in zipped {
        sum += a.abs_diff(*b);
    }
    return sum;
}

fn calculate_similarity(set1: &Vec<u32>, set2: &Vec<u32>) -> u32 {
    let mut score: u32 = 0;

    for value in set1 {
        let count = set2.iter().filter(|x| **x == *value).count() as u32;
        score += value * count;
    }

    return score;
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_sum() {
        let path = PathBuf::from("./resources/day1_test.txt");
        let sets = parse_input(path);
        assert_eq!(calculate_sum_of_distances(sets.0, sets.1), 11);
    }

    #[test]
    fn test_similarity() {
        let path = PathBuf::from("./resources/day1_test.txt");
        let sets = parse_input(path);
        assert_eq!(calculate_similarity(&sets.0, &sets.1), 31);
    }
}
pub fn solve(path: std::path::PathBuf) {
    let sum = calculate_sum_of_distances(path);
    println!(" The sum is {:?}", sum)
}

fn calculate_sum_of_distances(path: std::path::PathBuf) -> u32 {
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


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test() {
        let path = PathBuf::from("./resources/day1_test.txt");
        assert_eq!(calculate_sum_of_distances(path), 11);
    }
}
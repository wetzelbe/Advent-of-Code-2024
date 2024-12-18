use std::collections::HashMap;

use ndarray::Array2;

pub fn solve(path: std::path::PathBuf) {
    let input = parse_input(path);
    println!("The solution to part 1 is {}", solve_part1(&input));
    println!("The solution to part 2 is {}", solve_part2(&input));
}

fn parse_input(path: std::path::PathBuf) -> Array2<char> {
    let content = std::fs::read_to_string(&path).expect("could not read file");
    
    let h = content.lines().count();
    let mut value: Vec<char> = Vec::new();
    let mut w: usize = 0;

    for line in content.lines() {
        value.append(&mut line.chars().collect());
        w = line.len();
    }
    return ndarray::Array::from_shape_vec((h,w),value).expect("Could not put to matrix")
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    a: usize,
    b: usize,
    direction: char
}

fn get_neighbour_nodes(input: &Array2<char>, node: &Node) -> Vec<Node> {
    let mut neighbours = Vec::new();

    if node.direction == '>' || node.direction == '<' {
        neighbours.push(Node { a: node.a, b: node.b, direction: 'v' });
        neighbours.push(Node { a: node.a, b: node.b, direction: '^' });
    }
    if node.direction == 'v' || node.direction == '^' {
        neighbours.push(Node { a: node.a, b: node.b, direction: '>' });
        neighbours.push(Node { a: node.a, b: node.b, direction: '<' });
    }

    if input[[node.a, node.b + 1]] != '#' && node.direction == '>' {
        neighbours.push(Node { a: node.a, b: node.b + 1, direction: node.direction });
    }
    if input[[node.a, node.b - 1]] != '#' && node.direction == '<' {
        neighbours.push(Node { a: node.a, b: node.b - 1, direction: node.direction });
    }
    if input[[node.a + 1, node.b]] != '#' && node.direction == 'v' {
        neighbours.push(Node { a: node.a + 1, b: node.b, direction: node.direction });
    }
    if input[[node.a - 1, node.b]] != '#' && node.direction == '^' {
        neighbours.push(Node { a: node.a - 1, b: node.b, direction: node.direction });
    }

    return neighbours;
}

fn get_start_node(input: &Array2<char>) -> Node {
    for a in 0..input.dim().0 {
        for b in 0..input.dim().1 {
            if input[[a,b]] == 'S' {
                return Node {
                    a,b,
                    direction: '>'
                }
            }
        }
    }
    panic!("Did not find start node");
}

fn get_end_node(input: &Array2<char>) -> Node {
    for a in 0..input.dim().0 {
        for b in 0..input.dim().1 {
            if input[[a,b]] == 'E' {
                return Node {
                    a,b,
                    direction: '>'
                }
            }
        }
    }
    panic!("Did not find start node");
}

fn pop_smallest_distance(distances: &mut HashMap<Node, usize>, known_nodes: &mut Vec<Node>) -> Node {
    let mut min_distance: usize = usize::max_value();
    let mut min_node = known_nodes.first().expect("Error").clone();
    let mut min_i = 0;
    for (i, node) in known_nodes.iter().enumerate() {
        if distances.contains_key(node) && *distances.get(node).expect("Error") < min_distance {
            min_node = node.clone();
            min_i = i;
            min_distance = *distances.get(node).expect("Error");
        }
    }

    known_nodes.remove(min_i);

    return min_node;
}

fn update(distances: &mut HashMap<Node, usize>, pre: &mut HashMap<Node, Node>, u: &Node, v: &Node) {
    let mut new_distance = *distances.get(u).expect("Error");
    if u.direction != v.direction {
        new_distance += 1000;
    } else {
        new_distance += 1;
    }

    if new_distance < *distances.get(v).expect("Error") {
        distances.insert(v.clone(), new_distance);
        pre.insert(v.clone(), u.clone());
    }
}

fn solve_part1(input: &Array2<char>) -> usize {
    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut pre: HashMap<Node, Node> = HashMap::new();

    let mut known_nodes: Vec<Node> = Vec::new();

    let s = get_start_node(input);
    distances.insert(s.clone(), 0);
    known_nodes.push(s.clone());

    while known_nodes.len() > 0 {
        let u = pop_smallest_distance(&mut distances, &mut known_nodes);
        let end_node = get_end_node(input);

        if u.a == end_node.a && u.b == end_node.b {
            return *distances.get(&u).expect("Error");
        }

        for v in get_neighbour_nodes(input, &u) {
            if !distances.contains_key(&v) {
                known_nodes.push(v.clone());
                distances.insert(v.clone(), usize::max_value());
            }
            if known_nodes.contains(&v) {
                update(&mut distances, &mut pre, &u, &v);
            }
        }
    }

    return 0;
}

fn update2(distances: &mut HashMap<Node, usize>, pre: &mut HashMap<Node, Vec<Node>>, u: &Node, v: &Node) {
    let mut new_distance = *distances.get(u).expect("Error");
    if u.direction != v.direction {
        new_distance += 1000;
    } else {
        new_distance += 1;
    }

    if new_distance < *distances.get(v).expect("Error") {
        distances.insert(v.clone(), new_distance);
        pre.insert(v.clone(), vec![u.clone()]);
    } else if new_distance == *distances.get(v).expect("Error") {
        if pre.contains_key(&v.clone()) {
            let mut existing = pre.get(&v.clone()).expect("Error").clone();
            existing.push(u.clone());
            pre.insert(v.clone(), existing);
        } else {
            pre.insert(v.clone(), vec![u.clone()]);
        }
    }
}

fn recurse_fill(input: &mut Array2<char>, pre: &HashMap<Node, Vec<Node>>, node: &Node) {
    input[[node.a, node.b]] = 'O';

    if pre.contains_key(node) {
        let predecessors = pre.get(node).expect("Error");
        for predecessor in predecessors {
            recurse_fill(input, pre, predecessor);
        }
    }
    
}

fn solve_part2(input: &Array2<char>) -> usize {
    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut pre: HashMap<Node, Vec<Node>> = HashMap::new();

    let mut known_nodes: Vec<Node> = Vec::new();

    let s = get_start_node(input);
    distances.insert(s.clone(), 0);
    known_nodes.push(s.clone());

    let mut editable = input.clone();

    while known_nodes.len() > 0 {
        let u = pop_smallest_distance(&mut distances, &mut known_nodes);
        let end_node = get_end_node(input);

        if u.a == end_node.a && u.b == end_node.b {
            recurse_fill(&mut editable, &pre, &u);


            break;
        }

        for v in get_neighbour_nodes(input, &u) {
            if !distances.contains_key(&v) {
                known_nodes.push(v.clone());
                distances.insert(v.clone(), usize::max_value());
            }
            if known_nodes.contains(&v) {
                update2(&mut distances, &mut pre, &u, &v);
            }
        }
    }

    println!("{:#}", editable);

    return editable.flatten().iter().filter(|c| **c == 'O').count();
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_input(PathBuf::from("./resources/day16_test.txt"));
        assert_eq!(solve_part1(&input), 7036);
    }
    
    #[test]
    fn test_part1_2() {
        let input = parse_input(PathBuf::from("./resources/day16_test_2.txt"));
        assert_eq!(solve_part1(&input), 11048);
    }

    
    #[test]
    fn test_part2() {
        let input = parse_input(PathBuf::from("./resources/day16_test.txt"));
        assert_eq!(solve_part2(&input), 45);
    }
    
    #[test]
    fn test_part2_2() {
        let input = parse_input(PathBuf::from("./resources/day16_test_2.txt"));
        assert_eq!(solve_part2(&input), 64);
    }
}
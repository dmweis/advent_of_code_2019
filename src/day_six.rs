use std::fs;
use std::collections::HashMap;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| line.split(")"))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .collect()
}

fn edges_to_graph<'a>(edges: &'a Vec<(&str, &str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph: HashMap<_, Vec<&str>> = HashMap::new();
    for (parent, child) in edges {
        graph.entry(*parent).or_default().push(child);
    }
    graph
}

fn count_orbits(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    count_orbits_for_key(graph, "COM", -1)
}

fn count_orbits_for_key(graph: &HashMap<&str, Vec<&str>>, node: &str, count: i32) -> i32 {
    let mut counter = count+1;
    if let Some(children) = graph.get(node) {
        for child in children {
            counter += count_orbits_for_key(graph, child, count+1);
        }
    }
    counter
}

pub fn one() {
    let text = read_file("input/day_six.txt");
    let data = parse_input(&text);
    let graph = edges_to_graph(&data);
    let count = count_orbits(&graph);
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn parse_input_simple_tuple() {
        let test_input = "A)B\nB)C";
        let parsed = parse_input(test_input);
        let mut parsed_iter = parsed.iter();
        assert_eq!(parsed_iter.next(), Some(&("A", "B")));
        assert_eq!(parsed_iter.next(), Some(&("B", "C")));
    }

    #[test]
    fn orbit_counter_test() {
        let mut graph = HashMap::new();
        graph.insert("COM", vec!["A"]);
        graph.insert("A", vec!["B", "C"]);
        let orbit_count = count_orbits(&graph);
        assert_eq!(orbit_count, 5);
    }

    #[test]
    fn orbit_parse_count_test() {
        let text_input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let parsed = parse_input(text_input);
        let graph = edges_to_graph(&parsed);
        let count = count_orbits(&graph);
        assert_eq!(count, 42);
    }

    #[test]
    fn day_6_task_1() {
        let text = read_file("input/day_six.txt");
        let data = parse_input(&text);
        let graph = edges_to_graph(&data);
        let count = count_orbits(&graph);
        assert_eq!(count, 147807);
    }
}

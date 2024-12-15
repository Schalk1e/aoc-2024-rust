use ::std::collections::{HashMap, VecDeque};
use ::std::error::Error;
use ::std::fs;

fn get_rules(path: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let input = fs::read_to_string(path)?;
    let mut rules = Vec::new();

    for line in input.lines() {
        let pair = line
            .split("|")
            .map(str::parse::<i32>)
            // Collect OK values into vector and propagate an error <Result<Vec<_>, _>>
            .collect::<Result<Vec<_>, _>>()?;

        rules.push(pair);
    }

    Ok(rules)
}

fn compute_in_degrees(edges: Vec<Vec<i32>>) -> HashMap<i32, i32> {
    let mut in_degrees = HashMap::new();

    for edge in edges {
        if edge.len() != 2 {
            panic!("Invalid edge format: {:?}", edge); // Ensure all edges have exactly two elements
        }
        let source = edge[0];
        let target = edge[1];

        // Increment in-degrees for the target node
        let _ = *in_degrees.entry(source).or_insert(0); // Ensure the source node exists with in-degree 0
        *in_degrees.entry(target).or_insert(0) += 1; // Increment the in-degree of the target node
    }

    in_degrees
}

fn find_zero_in_degree_nodes(pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let degrees = compute_in_degrees(pairs);

    degrees
        .iter()
        .filter_map(|(key, value)| if *value == 0 { Some(key) } else { None })
        // This will simply clone the references into actual values.
        // Probably a better way of doing this.
        .cloned()
        .collect()
}

fn find_neighbours(pairs: &[Vec<i32>], node: i32) -> Vec<i32> {
    pairs
        .iter()
        // Remember this filter_map method. Common in the API.
        .filter_map(|v| if v[0] == node { Some(v[1]) } else { None })
        .collect()
}

fn sort_rules(pairs: Vec<Vec<i32>>) {
    // Need to build single ordered list from order rules.
    // This is the same as finding a topological ordering of a directed acyclical graph.
    // This can be done using Kahn's algorithm
    let mut q: VecDeque<i32> = VecDeque::new();
    let mut sorted: Vec<i32> = Vec::new();

    let zero_in_degree_nodes = find_zero_in_degree_nodes(pairs.clone());
    let mut in_degrees = compute_in_degrees(pairs.clone());

    // Initialise queue with zero in-degree nodes
    for node in zero_in_degree_nodes {
        // VecDequeue's have push_back and pop_front for FIFO operations.
        q.push_back(node)
    }

    while !q.is_empty() {
        // Pop key off queue
        let key = q.pop_front();
        // Push key to sorted list
        sorted.push(key.expect("REASON"));
        // Reduce in degrees of neighbours
        let neighbours = find_neighbours(&pairs, key.expect("REASON"));
        for neighbour in &neighbours {
            in_degrees.entry(*neighbour).and_modify(|value| *value -= 1);
            // Get in degree of neighbour after decrement
            let in_degree: i32 = *in_degrees.get(neighbour).unwrap();
            // If it's 0, push it to queue.
            if in_degree == 0 {
                q.push_back(*neighbour);
            };
        }
    }

    println!("{:?}", sorted);
}

pub fn part1() {
    let rules = get_rules("src/data/day5_rules.txt".to_string());

    sort_rules(rules.expect("REASON"));
}

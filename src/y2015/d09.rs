use std::collections::{HashMap, HashSet, VecDeque};

struct QItem {
    current: String,
    visited: HashSet<String>,
    dist: i64,
}

fn parse_graph(input: &str) -> HashMap<String, HashMap<String, i64>> {
    let mut distances: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        match tokens.as_slice() {
            [from, "to", to, "=", dist] => {
                let dist = dist.parse::<i64>().expect("failed to parse distance");
                distances.entry(from.to_string()).or_default().insert(to.to_string(), dist);
                distances.entry(to.to_string()).or_default().insert(from.to_string(), dist);
            },
            _ => panic!("invalid line: {}", line),
        }
    }

    distances
}

fn traverse<F: FnMut(i64)>(distances: HashMap<String, HashMap<String, i64>>, mut f: F) {
    let mut q = VecDeque::new();
    for k in distances.keys() {
        let mut visited = HashSet::new();
        visited.insert(k.to_string());
        q.push_back(QItem { current: k.to_string(), dist: 0, visited});
    }

    while let Some(i) = q.pop_front() {
        if i.visited.len() == distances.len() { // traversed all nodes, exit
            f(i.dist);
            continue;
        }

        for (k, v) in distances.get(&i.current).expect("No destination found") {
            if i.visited.contains(k) {
                continue;
            }

            let mut new_visited = i.visited.clone();
            new_visited.insert(k.to_string());
            q.push_back(QItem { current: k.to_string(), visited: new_visited, dist: i.dist + v });
        }
    }
}

pub fn p1(input: &str) -> i64 {
    let mut result = i64::MAX;
    traverse(parse_graph(input), |new_min| { result = result.min(new_min); });

    result
}

pub fn p2(input: &str) -> i64 {
    let mut result = i64::MIN;
    traverse(parse_graph(input), |new_max| { result = result.max(new_max); });

    result
}

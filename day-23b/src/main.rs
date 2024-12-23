use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();

    let edges: Vec<(String, String)> = lines
        .map(|line| {
            let line = line.as_ref().unwrap();
            let (a, b) = line.split_once("-").unwrap();

            (a.to_string(), b.to_string())
        })
        .collect();

    let mut graph = HashMap::new();

    for (a, b) in &edges {
        let edges = graph.entry(a.clone()).or_insert(HashSet::new());
        edges.insert(b.clone());

        let edges = graph.entry(b.clone()).or_insert(HashSet::new());
        edges.insert(a.clone());
    }

    let mut last_cliques = Vec::new();

    for i in 3..100 {
        let mut cliques = Vec::new();
        let mut frontier = VecDeque::new();
        frontier.push_back((Vec::new(), 0));

        while let Some((path, skip)) = frontier.pop_back() {
            if path.len() == i {
                cliques.push(path);
                continue;
            }

            for (i, (b, _)) in graph.iter().enumerate().skip(skip) {
                let not_connected = path.iter().any(|a| !graph[a].contains(b));
                if not_connected {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(b.to_string());
                frontier.push_back((new_path, i + 1));
            }
        }

        if cliques.is_empty() {
            break;
        }

        last_cliques = cliques.clone();
    }

    let mut last_clique = last_cliques.first().unwrap().clone();
    last_clique.sort();

    println!("{}", last_clique.join(","));
}

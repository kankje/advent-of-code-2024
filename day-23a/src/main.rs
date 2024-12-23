use std::collections::{HashMap, HashSet};
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

    let mut triangles = HashSet::new();

    for (i, (a, a_edges)) in graph.iter().enumerate() {
        for (j, (b, b_edges)) in graph.iter().enumerate().skip(i + 1) {
            if !a_edges.contains(b) {
                continue;
            }

            for (c, _) in graph.iter().skip(j + 1) {
                if !a_edges.contains(c) || !b_edges.contains(c) {
                    continue;
                }

                triangles.insert((a, b, c));
            }
        }
    }

    let count = triangles
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count();

    println!("{}", count);
}

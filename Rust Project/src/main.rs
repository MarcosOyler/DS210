use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct Graph {
    pub adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Graph> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut adjacency_list = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let from = parts[0].parse::<i32>().unwrap();
                let to = parts[1].parse::<i32>().unwrap();
                adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
            }
        }

        Ok(Graph { adjacency_list })
    }

    pub fn get_neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.adjacency_list.get(&vertex)
    }
}

fn compute_distances_bfs(start: i32, graph: &Graph) -> HashMap<i32, u32> {
    let mut distance = HashMap::new();
    distance.insert(start, 0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        if let Some(neighbors) = graph.get_neighbors(v) {
            for &u in neighbors {
                if !distance.contains_key(&u) {
                    distance.insert(u, distance[&v] + 1);
                    queue.push_back(u);
                }
            }
        }
    }

    distance
}

fn compute_all_distances(graph: &Graph) -> HashMap<i32, HashMap<i32, u32>> {
    let mut all_distances = HashMap::new();

    for &vertex in graph.adjacency_list.keys() {
        let distances = compute_distances_bfs(vertex, graph);
        all_distances.insert(vertex, distances);
    }

    all_distances
}
fn total_unique_nodes(graph: &Graph) -> usize {
    let mut unique_nodes = std::collections::HashSet::new();
    
    for (&key, values) in graph.adjacency_list.iter() {
        unique_nodes.insert(key); // Insert the key
        for &value in values {
            unique_nodes.insert(value); // Insert each value in the adjacency list
        }
    }

    unique_nodes.len()
}

fn main() {
    let graph = Graph::new("facebook_combined.txt").unwrap();

    println!("Adjacency list length: {}", graph.adjacency_list.len());

    println!("{}", total_unique_nodes(&graph));

    // Compute distances for all vertices
    let all_distances = compute_all_distances(&graph);

    println!("Sample adjacency list entries:");
    for (vertex, neighbors) in graph.adjacency_list.iter().take(5) {
        println!("Vertex {}: {:?}", vertex, neighbors);
    }

    // Print a sample of the distances from a few vertices to keep the output small
    println!("Sample distance outputs for multiple vertices:");
    for (vertex, distances) in all_distances.iter().take(3) {
        println!("Distances from vertex {}:", vertex);
        for (target, distance) in distances.iter().take(5) {
            println!("   To vertex {}: {}", target, distance);
        }
        println!("---");
    }
}

use std::collections::HashMap;
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
                
                // Ensure that each vertex records both ends of the connection
                adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
                adjacency_list.entry(to).or_insert_with(Vec::new).push(from);
            }
        }

        Ok(Graph { adjacency_list })
    }

    pub fn get_neighbors(&self, vertex: i32) -> Option<&Vec<i32>> {
        self.adjacency_list.get(&vertex)
    }
}

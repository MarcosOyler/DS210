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




fn analyze_six_degrees(all_distances: &HashMap<i32, HashMap<i32, u32>>) -> (HashMap<i32, usize>, f64) {
   let num_vertices = all_distances.len() as f64;
   let mut within_six_degrees = HashMap::new();
   let mut total_within_six = 0;


   for (vertex, distances) in all_distances {
       let count = distances.values().filter(|&&d| d <= 6 && d > 0).count();
       within_six_degrees.insert(*vertex, count);
       total_within_six += count;
   }


   let average_percentage = total_within_six as f64 / num_vertices / num_vertices * 100.0;


   (within_six_degrees, average_percentage)
}
fn largest_web_within_six_degrees(all_distances: &HashMap<i32, HashMap<i32, u32>>) -> usize {
   let mut max_web_size = 0;


   // Iterate over each node to determine the size of the "web" it forms with others within 6 degrees
   for (_node, distances) in all_distances {
       let web_size = distances.iter()
                               .filter(|&(_, &dist)| dist <= 6)  // Only consider nodes within 6 degrees
                               .count();


       if web_size > max_web_size {
           max_web_size = web_size;
       }
   }


   max_web_size
}
fn analyze_graph(graph: &Graph) {
   let all_distances = compute_all_distances(graph);


   // Compute various statistics
   let mut total_connections = 0;
   let mut connection_counts = HashMap::new();
   let mut distance_counts = HashMap::new();
   let mut total_distance = 0;
   let mut num_distances = 0;


   for (vertex, distances) in &all_distances {
       connection_counts.insert(*vertex, distances.len());
       total_connections += distances.len();


       for &distance in distances.values() {
           *distance_counts.entry(distance).or_insert(0) += 1;
           total_distance += distance as usize;
           num_distances += 1;
       }
   }

   let avg_connections = total_connections as f64 / all_distances.len() as f64;
   let mean_distance = total_distance as f64 / num_distances as f64;
   let mode_distance = find_mode(&distance_counts);

   println!("Average number of connections per vertex: {:.2}", avg_connections);
   println!("Mean distance between vertices: {:.2}", mean_distance);
   println!("Mode distance between vertices: {:?}", mode_distance);
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
   let (six_degrees_count, avg_percent) = analyze_six_degrees(&all_distances);


   println!("Individual vertex connectivity within 6 degrees:");
   for (vertex, count) in six_degrees_count.iter().take(5) {
       println!("Vertex {}: {} vertices within 6 degrees", vertex, count);
   }
   println!("Average percentage of vertices within 6 degrees of any vertex: {:.2}%", avg_percent);
   let max_web_size = largest_web_within_six_degrees(&all_distances);
   println!("The largest web size within 6 degrees is: {}", max_web_size);
   analyze_graph(&graph);
}



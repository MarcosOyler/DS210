use std::collections::{HashMap, VecDeque};
mod graph;
use graph::Graph; 
use std::io::{self, Write};

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

fn compute_distances_from_vertex(start: i32, graph: &Graph) -> HashMap<i32, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);
/*  The below uses a while loop that assigns the variable current to whatever vertex is popped from 
the front */
    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];
        if let Some(neighbors) = graph.get_neighbors(current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    distances.insert(neighbor, current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distances
}

/// Computes all pairwise distances in the graph using BFS for each vertex.
fn compute_all_distances(graph: &Graph) -> HashMap<i32, HashMap<i32, u32>> {
    let mut all_distances = HashMap::new();

    for &vertex in graph.adjacency_list.keys() {
        let distances = compute_distances_from_vertex(vertex, graph);
        all_distances.insert(vertex, distances);
    }

    all_distances
}

/// Calculates the number of connections each vertex has.
fn calculate_connection_counts(distances: &HashMap<i32, HashMap<i32, u32>>) -> HashMap<i32, usize> {
    distances.iter().map(|(&vertex, dist_map)| (vertex, dist_map.len())).collect()
}

/// Calculates and returns the largest web size within 6 degrees and the corresponding vertex.
fn find_largest_web_within_six_degrees(distances: &HashMap<i32, HashMap<i32, u32>>) -> (Option<i32>, usize) {
    let mut max_web_size = 0;
    let mut vertex_with_max_web = None;

    for (&vertex, dist_map) in distances {
        let web_size = dist_map.values().filter(|&&d| d <= 6).count();
        if web_size > max_web_size {
            max_web_size = web_size;
            vertex_with_max_web = Some(vertex);
        }
    }

    (vertex_with_max_web, max_web_size)
}

/// Identifies the mode of the connection counts.
fn find_mode_vertex(connection_counts: &HashMap<i32, usize>) -> Option<usize> {
    let mut frequency = HashMap::new();
    let mut max_freq = 0;
    let mut mode_vertex = None;

    for &count in connection_counts.values() {
        let count_freq = frequency.entry(count).or_insert(0);
        *count_freq += 1;

        if *count_freq > max_freq {
            max_freq = *count_freq;
            mode_vertex = Some(count);
        }
    }

    mode_vertex
}
pub fn analyze_graph(graph: &Graph) {
    let all_distances = compute_all_distances(graph);
    let connection_counts = calculate_connection_counts(&all_distances);
    let (vertex_with_max_web, max_web_size) = find_largest_web_within_six_degrees(&all_distances);
    let mode_vertex = find_mode_vertex(&connection_counts);

    println!("Vertex with the largest web within 6 degrees: {:?}", vertex_with_max_web);
    println!("Size of the largest web within 6 degrees: {}", max_web_size);
    println!("Mode vertex by number of connections: {:?}", mode_vertex);
}

pub fn print_top_vertices_by_neighbors<W: Write>(graph: &Graph, top_n: usize, writer: &mut W) -> io::Result<()> {
    let mut neighbor_counts: Vec<(i32, usize)> = graph.adjacency_list.iter()
        .map(|(vertex, neighbors)| (*vertex, neighbors.len()))
        .collect();

    neighbor_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let top_vertices = neighbor_counts.iter().take(top_n);

    writeln!(writer, "Top {} vertices by number of neighbors:", top_n)?;
    for (vertex, count) in top_vertices {
        writeln!(writer, "Vertex {}: {} neighbors", vertex, count)?;
    }
    Ok(())
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
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::str;

    #[test]
    fn test_print_top_vertices_by_neighbors() {
        let mut graph = Graph {
            adjacency_list: HashMap::new(),
        };

        // Setup a simple graph
        graph.adjacency_list.insert(0, vec![1, 2, 3]);
        graph.adjacency_list.insert(1, vec![0, 2]);
        graph.adjacency_list.insert(2, vec![0, 1]);
        graph.adjacency_list.insert(3, vec![0]);

        let mut output = vec![];
        print_top_vertices_by_neighbors(&graph, 2, &mut output).unwrap();

        let output_str = str::from_utf8(&output).unwrap();
        assert!(output_str.contains("Top 2 vertices by number of neighbors:"));
        assert!(output_str.contains("Vertex 0: 3 neighbors"));
        assert!(output_str.contains("Vertex 1: 2 neighbors"));
    }

    #[test]
    fn test_compute_all_distances() {
        let mut graph = Graph {
            adjacency_list: HashMap::new(),
        };
        // Creating an undirected graph: 0 - 1 - 2 (both directions)
        graph.adjacency_list.insert(0, vec![1]);
        graph.adjacency_list.insert(1, vec![0, 2]);
        graph.adjacency_list.insert(2, vec![1]);

        let distances = compute_all_distances(&graph);

        assert_eq!(distances[&0][&1], 1); // Check distance from 0 to 1
        assert_eq!(distances[&1][&2], 1); // Check distance from 1 to 2
        assert_eq!(distances[&0][&2], 2); // Check distance from 0 to 2 through 1
        assert_eq!(distances[&2][&0], 2); // Check distance from 2 to 0 through 1
    }

    #[test]
    fn test_compute_distances_from_vertex() {
        let mut graph = Graph {
            adjacency_list: HashMap::new(),
        };
        // Creating an undirected graph: 0 - 1 - 2 - 3 (chain)
        graph.adjacency_list.insert(0, vec![1]);
        graph.adjacency_list.insert(1, vec![0, 2]);
        graph.adjacency_list.insert(2, vec![1, 3]);
        graph.adjacency_list.insert(3, vec![2]);

        let distances = compute_distances_from_vertex(0, &graph);

        assert_eq!(distances[&1], 1); // Direct neighbor
        assert_eq!(distances[&2], 2); // Two hops away
        assert_eq!(distances[&3], 3); // Three hops away
    }
}

fn main() {
   let graph = Graph::new("facebook_combined.txt").unwrap();
   let mut stdout = io::stdout();
   
  println!("Adjacency list length: {}", graph.adjacency_list.len());


   println!("Number of Nodes {}", total_unique_nodes(&graph));


   // Compute distances for all vertices
   let all_distances = compute_all_distances(&graph);


   println!("Sample adjacency list entries:");
   for (vertex, neighbors) in graph.adjacency_list.iter().take(5) {
       println!("Vertex {}: {:?}", vertex, neighbors);
   }
   print_top_vertices_by_neighbors(&graph, 5, &mut stdout).unwrap();


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
   analyze_graph(&graph); 
}



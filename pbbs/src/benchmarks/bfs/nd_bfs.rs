use num_traits::PrimInt; // For integer traits
use num_traits::Float;   // For float traits
use rayon::{prelude::*, vec};
use std::ops::Sub;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::DefInt;
use crate::common::graph::*;

pub fn bfs(source: usize, g: &Graph, parents: &mut Vec<i32>, verbose: bool) -> (usize, usize) {
    let n = g.num_vertices();
    let m = g.num_edges();
    
    // Initialize data structures
    let mut visited: Vec<AtomicBool> = (0..n)
        .map(|_| AtomicBool::new(false))
        .collect();
    let mut frontier = vec![source];
    
    // Mark source as visited
    visited[source].store(true, Ordering::SeqCst);
    let mut total_visited = 0;
    let mut round = 0;

    // Continue while frontier is not empty
    while !frontier.is_empty() {
        total_visited += frontier.len();
        round += 1;

        // Calculate offsets for the next frontier
        let mut offsets: Vec<usize> = (0..frontier.len())
            .into_par_iter()
            .map(|i| g.index(frontier[i]).degree)
            .collect();

        // Prefix sum to get write positions
        let mut total = 0;
        for offset in &mut offsets {
            let curr = *offset;
            *offset = total;
            total += curr;
        }
        let total_size = total;

        // Allocate space for next frontier
        let mut frontier_next = vec![-1; total_size];

        // Process current frontier in parallel
        frontier
            .par_iter()
            .enumerate()
            .for_each(|(i, &v)| {
                let vertex = g.index(v);
                let offset = offsets[i];
                
                // Process each neighbor
                for (j, &ngh) in vertex.neighbors.iter().enumerate() {
                    let ngh = ngh as usize;
                    // Try to mark unvisited neighbors
                    if !visited[ngh].load(Ordering::Relaxed) &&
                       !visited[ngh].swap(true, Ordering::SeqCst) {
                        frontier_next[offset + j] = ngh as i32;
                        parents[ngh] = v as i32;
                    }
                }
            });

        // Filter out -1 entries for next frontier
        frontier = frontier_next
            .into_par_iter()
            .filter(|&x| x >= 0)
            .map(|x| x as usize)
            .collect();
    }

    // Set parent of source to itself
    parents[source] = source as i32;

    if verbose {
        println!("BFS completed in {} rounds", round);
        println!("Visited {} vertices", total_visited);
    }

    (total_visited, round)
}
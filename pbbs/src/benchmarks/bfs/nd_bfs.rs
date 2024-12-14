use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::graph::*;
use std::sync::{Arc, Mutex};

pub fn bfs(source: usize, g: &Graph, verbose: bool) -> (usize, usize) {
    let n = g.num_vertices();
    let _m = g.num_edges();
    
    let visited: Vec<AtomicBool> = (0..n)
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

        let mut offsets: Vec<usize> = (0..frontier.len())
            .into_par_iter()
            .map(|i| g.index(frontier[i]).degree)
            .collect();

        let mut total = 0;
        for offset in &mut offsets {
            let curr = *offset;
            *offset = total;
            total += curr;
        }
        let total_size = total;

        let frontier_next = vec![-1; total_size];

        let f_next = Arc::new(Mutex::new(frontier_next));

        frontier
            .par_iter()
            .enumerate()
            .for_each(|(i, &v)| {
                let vertex = g.index(v);
                let offset = offsets[i];
                
                for (j, &ngh) in vertex.neighbors.iter().enumerate() {
                    let ngh = ngh as usize;
                    if !visited[ngh].load(Ordering::Relaxed) &&
                       !visited[ngh].swap(true, Ordering::SeqCst) {
                        let mut fnxt = f_next.lock().unwrap();
                        fnxt[offset + j] = ngh as i32;
                    }
                }
            });

        frontier = f_next.lock().unwrap()
            .iter()
            .cloned()
            .filter(|&x| x >= 0)
            .map(|x| x as usize)
            .collect();
    }

    if verbose {
        println!("BFS completed in {} rounds", round);
        println!("Visited {} vertices", total_visited);
    }

    (total_visited, round)
}
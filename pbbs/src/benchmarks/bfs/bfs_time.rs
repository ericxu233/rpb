use core::panic;
use std::time::Duration;
use clap::Parser;
use rayon::prelude::*;
use std::collections::VecDeque;

#[path ="mod.rs"] mod bfs;
#[path ="../../misc.rs"] mod misc;
#[path ="../macros.rs"] mod macros;
#[path ="../../common/io.rs"] mod io;
#[path ="../../common/mod.rs"] mod common;
#[path ="../../common/graph.rs"] mod graph;
#[path ="../../common/graph_io.rs"] mod graph_io;

use misc::*;
use graph::Graph;
use graph_io::read_graph_from_file;
use io::write_slice_to_file_seq;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// the algorithm to use
    #[clap(short, long, value_parser, default_value_t = Algs::NAIVE)]
    algorithm: Algs,

    /// the output filename
    #[clap(short, long, required=false, default_value_t = ("").to_string())]
    ofname: String,

    /// the input filename
    #[clap(value_parser, required=true)]
    ifname: String,

    /// the number of rounds to execute the benchmark
    #[clap(short, long, value_parser, required=false, default_value_t=1)]
    rounds: usize,

    /// The source vertex to start BFS from
    #[clap(short = 's', long, value_parser, required = false, default_value_t = 0)]
    source: usize,

    /// Whether to print verbose output
    #[clap(short = 'v', long, required=false, default_value_t = false)]
    verbose: bool,

    /// Whether to check the results or not
    #[clap(short = 'c', long, required=false, default_value_t = false)]
    check: bool,
}

define_algs!(
    (NAIVE, "naive")
);

pub fn run(
    alg: Algs,
    rounds: usize, 
    g: &Graph,
    source: usize,
    verbose: bool
) -> (Vec<i32>, Duration) {
    let n = g.num_vertices();
    let mut parents = vec![-1; n];
    let parents_ptr = &parents as *const Vec<i32> as usize;

    let f = match alg {
        Algs::NAIVE => {bfs::nd_bfs::bfs}
    };

    let mean = time_loop(
        "bfs",
        rounds,
        Duration::new(1, 0),
        || { unsafe { *(parents_ptr as *mut Vec<i32>).as_mut().unwrap() = vec![-1; n]; } },
        || { f(source, g, &mut parents, verbose); },
        || {},
    );

    if verbose {
        let visited = parents.par_iter().filter(|&&x| x != -1).count();
        println!("total visited = {}", visited);
    }

    (parents, mean)
}

fn check(g: &Graph, parents: &[i32], source: usize) -> bool {
    let n = g.num_vertices();
    
    // Check basic properties
    if parents.len() != n {
        println!("Parent array size mismatch");
        return false;
    }

    // Check source vertex
    if parents[source] != source as i32 {
        println!("Source vertex doesn't point to itself");
        return false;
    }

    // Do a sequential BFS to get correct parent array
    let mut correct_parents = vec![-1; n];
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];

    // Initialize with source
    queue.push_back(source);
    visited[source] = true;
    correct_parents[source] = source as i32;

    // Standard BFS
    while let Some(v) = queue.pop_front() {
        let vertex = g.index(v);
        for &u in vertex.neighbors {
            let u = u as usize;
            if !visited[u] {
                visited[u] = true;
                queue.push_back(u);
                correct_parents[u] = v as i32;
            }
        }
    }

    // Compare results
    let mut is_correct = true;
    for i in 0..n {
        if parents[i] != correct_parents[i] {
            println!("Mismatch at vertex {}: got {}, expected {}", 
                    i, parents[i], correct_parents[i]);
            is_correct = false;
        }
    }

    // Check that parent relationships form valid paths to source
    for i in 0..n {
        if parents[i] != -1 {
            // Check path from i to source
            let mut current = i;
            let mut path_length = 0;
            let max_path_length = n; // No path should be longer than n
            
            while current != source && path_length < max_path_length {
                let parent = parents[current] as usize;
                
                // Verify edge exists in graph
                let vertex = g.index(parent);
                if !vertex.neighbors.contains(&(current as u32)) {
                    println!("Invalid edge in parent relationship: {} -> {}", parent, current);
                    return false;
                }
                
                current = parent;
                path_length += 1;
            }
            
            if path_length >= max_path_length {
                println!("Cycle detected in parent relationships");
                return false;
            }
        }
    }

    is_correct
}

fn main() {
    init!();
    let args = Args::parse();

    // Read graph from file
    let mut g = read_graph_from_file(&args.ifname);
    g.add_degrees();

    // Run BFS
    let (parents, duration) = run(
        args.algorithm,
        args.rounds,
        &g,
        args.source,
        args.verbose
    );

    // Check results if requested
    if args.check {
        let is_correct = check(&g, &parents, args.source);
        println!("Result is {}", is_correct);
    }

    // Write results if output file specified
    if !args.ofname.is_empty() {
        let result: Vec<String> = parents.iter()
            .map(|x| x.to_string())
            .collect();
        write_slice_to_file_seq(&result, &args.ofname);
    }

    println!("Runtime {:?}", duration);
}
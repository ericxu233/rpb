use num_traits::PrimInt; // For integer traits
use num_traits::Float;   // For float traits
use rayon::{prelude::*, vec};
use std::ops::Sub;
use crate::DefInt;
use crate::common::geometry::*;

// Declare `trait.rs` directly in main.rs
use crate::common::traits::Length;

pub fn ann(inp: &[Point2d<f64>], k: usize, res: &mut Vec<Vec<usize>>)
{
    // does not support for k > 1
    if k > 1 {
        panic!("k > 1 is not supported");
    }

    let n = inp.len();

    // Calculate nearest neighbors without mutating `inp`
    res
        .par_iter_mut() // Parallel mutable iterator over `nearest_indices`
        .enumerate()
        .for_each(|(i, index)| {
            let mut current_nearest_index = (i + 1) % n;
            let mut min_distance = (inp[i].clone() - inp[current_nearest_index].clone()).length();

            for j in 0..n {
                if j != i {
                    let dist = (inp[i].clone() - inp[j].clone()).length();
                    if dist < min_distance {
                        current_nearest_index = j;
                        min_distance = dist;
                    }
                }
            }

            // Store the nearest neighbor index for the current element
            index[0] = current_nearest_index;
        });
}

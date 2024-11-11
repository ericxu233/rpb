use num_traits::PrimInt;
use rayon::{prelude::*, vec};
use std::ops::Sub;
use crate::DefInt;

// Declare `trait.rs` directly in main.rs
#[path = "traits.rs"] mod traits;
use traits::{HasPoint, Length};

pub fn ann<const MAXK: usize, VTX>(inp: &mut [VTX], k: usize, res: &mut Vec<Vec<usize>>)
where
    VTX: HasPoint + Sync + Send + Clone,
    VTX::PointType: Copy + Sub<Output = VTX::PointType> + Length,
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
            let mut min_distance = (inp[i].pt().clone() - inp[current_nearest_index].pt().clone()).length();

            for j in 0..n {
                if j != i {
                    let dist = (inp[i].pt().clone() - inp[j].pt().clone()).length();
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

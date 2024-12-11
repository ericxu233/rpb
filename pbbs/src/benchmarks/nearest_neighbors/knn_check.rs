#![feature(slice_partition_dedup)]
// ============================================================================
// This code is part of RPB.
// ----------------------------------------------------------------------------
// MIT License
//
// Copyright (c) 2024-present Javad Abdi, Hanyang (Eric) Xu, Mark C. Jeffrey
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// ============================================================================

use clap::Parser;
use rayon::prelude::*;

#[path ="mod.rs"] mod knn;
#[path ="../../misc.rs"] mod misc;
#[path ="../macros.rs"] mod macros;
#[path ="../../common/io.rs"] mod io;
#[path ="../../common/mod.rs"] mod common;

use misc::*;
use knn::{naive};
use io::{read_big_file_to_vec, read_file_to_vec, write_slice_to_file_seq};
use common::geometry_io::{read_points2d_from_file, read_points3d_from_file};
use common::geometry::*;

// Declare `trait.rs` directly in main.rs
use crate::common::traits::Length;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// BW results filename
    #[clap(value_parser, required=true)]
    rfname: String,

    /// the input graph's filename
    #[clap(value_parser, required=true)]
    ifname: String,

    /// the number of nearest neighbors to find
    #[clap(short, long, value_parser, required=false, default_value_t=0)]
    k: usize,
}

// niave knn
fn check(inp: &mut [Point2d<f64>], out: Vec<Vec<usize>>, k: usize) -> bool {
    assert_eq!(out.len(), k*inp.len());

    // compute knn for inp
    let mut knn = Vec::with_capacity(inp.len());
    for p in inp.iter() {
        let mut knn_p = Vec::with_capacity(k);
        for q in inp.iter() {
            if knn_p.len() < k {
                knn_p.push(q);
            } else {
                let mut max_dist = 0.0;
                let mut max_idx = 0;
                for (i, r) in knn_p.iter().enumerate() {
                    let dist = (p.clone() - *r.clone()).length();
                    if dist > max_dist {
                        max_dist = dist;
                        max_idx = i;
                    }
                }
                if (p.clone() - q.clone()).length() < max_dist {
                    knn_p[max_idx] = q;
                }
            }
        }
        knn.push(knn_p);
    }

    // compare knn with out
    let mut diff_count = 0usize;
    for i in 0..inp.len() {
        for j in 0..k {
            if knn[i][j] != out[i*k+j] {
                diff_count+=1;
            }
        }
    }

    diff_count == 0
}

fn main() {
    let args = Args::parse();
    let mut inp = read_points2d_from_file(&args.ifname);
    let mut out = read_file_to_vec<usize>(&args.rfname);
    let k = args.k;

    let res = check(&mut inp, &mut out, k);
    println!("Result is {}", res);
}


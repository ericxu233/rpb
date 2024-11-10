use std::time::Duration;
// ============================================================================
// This code is part of RPB.
// ----------------------------------------------------------------------------
// MIT License
//
// Copyright (c) 2023-present Javad Abdi, Mark C. Jeffrey, Hanyang (Eric) Xu
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
// do a hello world in rust

use clap::Parser;

#[path ="mod.rs"] mod hist;
#[path ="../../misc.rs"] mod misc;
#[path ="../macros.rs"] mod macros;
#[path ="../../common/io.rs"] mod io;
#[path ="../../common/mod.rs"] mod common;

use misc::*;
// use hist::{sequential, parallel};
use io::{read_big_file_to_vec, write_slice_to_file_seq};
use common::geometry_io::{read_points2d_from_file, read_points3d_from_file};
use common::geometry::{Point2d, Point3d};


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

    #[clap(short, long, value_parser, required=true)]
    buckets: usize,
}

define_algs!(
    (NAIVE, "naive"),
    (SEQUENTIAL, "sequential")
);

struct Vertex<PT, const KK: usize> {
    identifier: i32,
    pt: PT,  // the point itself, which could be either Point2d or Point3d
    ngh: [Option<Box<Vertex<PT, KK>>>; KK],  // Array of optional neighbors
    counter: usize,
    counter2: usize,
}

impl<PT, const KK: usize> Vertex<PT, KK> {
    fn new(pt: PT, identifier: i32) -> Self {
        Self {
            identifier,
            pt,
            ngh: std::array::from_fn(|_| None),
            counter: 0,
            counter2: 0,
        }
    }
}



fn main() {
    init!();
    let args = Args::parse();
}
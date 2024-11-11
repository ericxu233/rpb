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

#[path ="mod.rs"] mod knn;
#[path ="../../misc.rs"] mod misc;
#[path ="../macros.rs"] mod macros;
#[path ="../../common/io.rs"] mod io;
#[path ="../../common/mod.rs"] mod common;
#[path = "traits.rs"] mod traits;

use misc::*;
use knn::{naive};
use io::{read_big_file_to_vec, write_slice_to_file_seq};
use common::geometry_io::{read_points2d_from_file, read_points3d_from_file};
use common::geometry::{Point2d, Point3d};
use traits::{HasPoint, Length};
use std::ops::Sub;


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

    /// The number of nearest neighbors to find
    #[clap(short = 'k', long, value_parser, required = false, default_value_t = 1)]
    k: usize,

    /// The dimension (2 for 2D points, 3 for 3D points)
    #[clap(short = 'd', long, value_parser, required = false, default_value_t = 2)]
    dimension: usize
}

define_algs!(
    (NAIVE, "naive")
);

struct Vertex<PT> {
  identifier: i32,
  pt: PT,  // the point itself, which could be either Point2d or Point3d
  counter: usize,
  counter2: usize,
}

impl<PT> Vertex<PT> {
  fn new(pt: PT, identifier: i32) -> Self {
      Self {
          identifier,
          pt,
          counter: 0,
          counter2: 0,
      }
  }

  fn pt(&self) -> &PT {
      &self.pt
  }
}

pub fn run<T>(alg: Algs, rounds: usize, arr: &[Vertex<T>], k: usize) -> (Vec<Vec<usize>>, Duration)
where
    T: HasPoint + Sync + Send + Clone,
    T::PointType: Copy + Sub<Output = T::PointType> + Length,
{
    // Define the MAXK constant (you need to choose the value based on your use case)
    const MAXK: usize = 1;

    // Wrap `ann` in a closure with specified generics
    let f = match alg {
        Algs::NAIVE => |arr: &[Vertex<T>], k: usize, res: &mut Vec<Vec<usize>>| {
            naive::ann::<MAXK, Vertex<T>>(arr, k, res)
        },
    };

    let mut r = vec![];
    let r_ptr = &r as *const Vec<Vec<usize>> as usize;

    let mean = time_loop(
        "knn",
        rounds,
        Duration::new(1, 0),
        || { unsafe { *(r_ptr as *mut Vec<Vec<usize>>).as_mut().unwrap() = vec![]; } },
        || { f(arr, k, &mut r); },
        || {},
    );

    (r, mean)
}


fn main() {
    init!();
    let args = Args::parse();

    let ifname = args.ifname;
    let ofname = args.ofname;
    let k = args.k;
    let rounds = args.rounds;
    let dimension = args.dimension;

    if dimension != 2 && dimension != 3 {
        panic!("Only 2D and 3D points are supported");
    }

    if dimension == 2 {
      let points: Vec<Point2d<f64>> = read_points2d_from_file(&ifname);
    } else if dimension == 3 {
      let points: Vec<Point3d<f64>> = read_points3d_from_file(&ifname);
    }


}
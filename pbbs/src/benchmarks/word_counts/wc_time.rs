use clap::Parser;
use std::time::Duration;
#[path ="mod.rs"] mod wc;
#[path ="../../misc.rs"] mod misc;
#[path ="../macros.rs"] mod macros;
#[path ="../../common/io.rs"] mod io;

use misc::*;
use wc::{serial, parallel};
use io::{chars_from_file, write_slice_to_file_seq};


#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// the algorithm to use
    #[clap(short, long, value_parser, default_value_t = Algs::PARALLEL)]
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
}

define_algs!(
    (PARALLEL, "parallel"),
    (SEQUENTIAL, "sequential")
);

pub fn run(alg: Algs, rounds: usize, arr: Vec<u8>) -> (Vec<String>, Duration) {
    let f = match alg {
        Algs::PARALLEL => {parallel::wc},
        Algs::SEQUENTIAL => {serial::wc},
    };
    let mut res: Vec<(String, i64)> = vec![];
    let res_ptr = &res as *const Vec<(String, i64)> as usize;
    
    
    let mean = time_loop(
        "wc",
        rounds,
        Duration::new(1, 0),
        || { unsafe { *(res_ptr as *mut Vec<(String, i64)>).as_mut().unwrap() = vec![]; } },
        || { f(&mut arr.clone(), &mut res); },
        || {}
    );

    let res = res.iter().map(|(word, count)| format!("{} {}", word.to_string(), count.to_string())).collect();
    (res, mean)
}

fn main() {
    init!();
    let args: Args = Args::parse();
    let arr = chars_from_file(&args.ifname, false).unwrap();
    let (r, d) = run(args.algorithm, args.rounds, arr);

    finalize!(
        args,
        r,
        d,
        write_slice_to_file_seq(&r, args.ofname)
    );
}

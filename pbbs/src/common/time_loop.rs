use std::time::Duration;
// ============================================================================
// This code is part of RPB.
// ----------------------------------------------------------------------------
// MIT License
//
// Copyright (c) 2023-present Javad Abdi, Mark C. Jeffrey
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


use parlay::Timer;


/// Runs a function `runf` for `r` times and returns the mean time.
/// Before each run, `initf` is called and after each run `endf` is called.
/// `delay` is the minimum time of warm-up.
pub fn time_loop<S, W, T>(
    name: &str,
    r: usize,
    delay: Duration,
    mut initf: S,
    mut runf: W,
    mut endf: T
) -> Duration
where
    S: FnMut(),
    W: FnMut(),
    T: FnMut(),
{
    let mut t = Timer::new(name);
    let mut ot = Timer::new("OutLoopTime");

    t.start();
    while t.total_time() < delay { initf(); runf(); endf(); }

    ot.start();
    for _ in 0..r {
        initf();
        t.start();
        runf();
        t.next("");
        endf();
    }
    ot.stop();
    ot.total();

    t.total_time() / r as u32
}
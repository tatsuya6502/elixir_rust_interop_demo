// -*- coding:utf-8-unix -*-

// The MIT License
//
// Copyright (C) 2016 by Tatsuya Kawano <tatsuya@hibaridb.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::thread;

const MAX_THREADS: u32 = 64;

#[allow(dead_code)]
pub fn calc_pi(n: u32) -> Result<f64, String> {
    let w = 1.0 / (n as f64);
    let mut s = 0.0;
    for i in 0..n {
        let x = (i as f64) * w;
        s += (1.0 - x * x).sqrt();
    }
    Ok(4.0 * w * s)
}

#[allow(dead_code)]
pub fn calc_pi_parallel(n: u32, num_threads: u32) -> Result<f64, String> {
    if num_threads <= 0 || num_threads > MAX_THREADS {
        Err(format!("Invalid num_threads {}. It must be > 0 and <= {}",
                    num_threads, MAX_THREADS))
    } else if n % num_threads != 0 {
        Err(format!("n {} must be a multiple of num_threads {}",
                    n, num_threads))
    } else {
        let len = n / num_threads;
        let handles: Vec<_> = (0..num_threads).map(|i| {
            thread::spawn(move || {
                calc_pi_range(n, len * i, len)
            })
        }).collect();

        let results = handles.into_iter().map(|h| { h.join().unwrap() });
        // トレイト std::iter::Iterator sum() は Rust 1.5.0 では unstable に
        // 指定されており使えない。代わりに fold() を使う。
        let pi: f64 = results.into_iter().fold(0.0, |acc, p| { acc + p });
        Ok(pi)
    }
}

fn calc_pi_range(n: u32, offset: u32, count: u32) -> f64 {
    let w = 1.0 / (n as f64);
    let mut s = 0.0;
    for i in offset..(offset + count) {
        let x = (i as f64) * w;
        s += (1.0 - x * x).sqrt();
    }
    4.0 * w * s
}

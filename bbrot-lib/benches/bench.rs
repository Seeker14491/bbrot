#![feature(test)]

extern crate test;
extern crate buddhabrot;

use buddhabrot::Buddhabrot;
use test::{Bencher, black_box};

#[bench]
fn bench(b: &mut Bencher) {
    let size = 900;
    let buddhabrot = Buddhabrot {
        width: size,
        height: size,
        xfocus: -0.7,
        yfocus: 0.0,
        scale: size as FP / 2.6,
        path_count_min: 1_000_000, // 55_000_000
    };
    b.iter(|| black_box(buddhabrot.compute()));
}

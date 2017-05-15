extern crate buddhabrot;

use buddhabrot::Setup;

fn main() {
    Setup::<f32>::new(512, 512, 500_000)
        .save_to_png("out.png")
        .unwrap();
}

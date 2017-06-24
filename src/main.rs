#![recursion_limit = "1024"]

extern crate bbrot_lib;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

use bbrot_lib::Setup;
use clap::{App, Arg};
use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let matches = App::new("Buddhabrot Generator")
        .about("Generates a png image of a Buddhabrot")
        .version(crate_version!())
        .args_from_usage(
            "<WIDTH> 'Width of rendering'
             <HEIGHT> 'Height of rendering'
             <POINTS> 'Minimum number of initial points to iterate'"
        )
        .arg(
            Arg::with_name("PRECISION")
                .short("p")
                .long("precision")
                .default_value("64")
                .possible_values(&["32", "64"])
                .help("Whether to use 32 or 64 bit floating point numbers"),
        )
        .arg_from_usage("<OUTPUT> 'File to save output to'")
        .get_matches();

    let read_usize_arg = |arg| value_t!(matches.value_of(arg), usize).unwrap_or_else(|e| e.exit());
	let read_u64_arg = |arg| value_t!(matches.value_of(arg), u64).unwrap_or_else(|e| e.exit());

    let width = read_usize_arg("WIDTH");
    let height = read_usize_arg("HEIGHT");
    let points = read_u64_arg("POINTS");
    let output = matches.value_of("OUTPUT").unwrap();

    match matches.value_of("PRECISION").unwrap() {
        "32" => {
            Setup::<f32>::new(width, height, points)
                .save_to_png(output)
                .chain_err(|| "error writing png")
        }
        "64" => {
            Setup::<f64>::new(width, height, points)
                .save_to_png(output)
                .chain_err(|| "error writing png")
        }
        _ => unreachable!(),
    }
}

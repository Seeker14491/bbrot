#![feature(conservative_impl_trait, integer_atomics, try_from)]

#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts,
trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications)]

extern crate crossbeam;
extern crate image;
extern crate num;
extern crate num_cpus;
extern crate rand;

mod bucket_field;
mod setup;
mod path_iterator;
mod random_complex_generator;

pub use setup::Setup;

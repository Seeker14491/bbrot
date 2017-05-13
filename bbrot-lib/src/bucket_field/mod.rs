//! The atomic and non-atomic bucket field types.

mod atomic_bucket_field;
mod nonatomic_bucket_field;

pub use bucket_field::atomic_bucket_field::*;
pub use bucket_field::nonatomic_bucket_field::*;

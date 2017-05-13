// TODO: use Index trait?


use bucket_field::NonatomicBucketField;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct AtomicBucketField {
    pub field: Box<[AtomicU32]>,
    pub width: usize,
    pub height: usize,
}

impl AtomicBucketField {
    pub fn new(width: usize, height: usize) -> Self {
        AtomicBucketField {
            field: (0..(width * height))
                .map(|_| AtomicU32::new(0))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            width: width,
            height: height,
        }
    }

    pub fn hit(&self, x: usize, y: usize) {
        if (x < self.width) && (y < self.height) {
            // TODO: Check if this is generating bounds-checking code.
            let bucket = &self.field[y * self.width + x];
            bucket.store(bucket.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
        }
    }
}

impl From<NonatomicBucketField> for AtomicBucketField {
    fn from(x: NonatomicBucketField) -> Self {
        let field = x.field
            .iter()
            .map(|&x| AtomicU32::new(x))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        AtomicBucketField {
            field: field,
            width: x.width,
            height: x.height,
        }
    }
}

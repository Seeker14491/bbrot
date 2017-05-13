use bucket_field::AtomicBucketField;
use image::ColorType;
use image::png::PNGEncoder as PngEncoder;
use std::cmp;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::u8;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NonatomicBucketField {
    pub field: Box<[u32]>,
    pub width: usize,
    pub height: usize,
    pub hits: u64,
}

impl NonatomicBucketField {
    pub fn new(width: usize, height: usize) -> Self {
        NonatomicBucketField {
            field: (0..(width * height))
                .map(|_| 0)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            width: width,
            height: height,
            hits: 0,
        }
    }

    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let factor = 2.0 * (self.height * self.width) as f64 / self.hits as f64;

        // TODO: write proper mapping closure
        let pixels: Vec<_> = self.field
            .iter()
            .map(|&x| cmp::min((x as f64 * factor).powi(2) as u32, u8::MAX as u32) as u8)
            .collect();
        PngEncoder::new(try!(File::create(apply_extension(path, "png")))).encode(&pixels,
                                                                                 self.width as u32,
                                                                                 self.height as u32,
                                                                                 ColorType::Gray(8))
    }
}

impl From<AtomicBucketField> for NonatomicBucketField {
    fn from(x: AtomicBucketField) -> Self {
        let field = x.field
            .iter()
            .map(|x| x.load(Ordering::Relaxed))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let hits = field.iter().fold(0, |acc, &elem| elem as u64 + acc);
        NonatomicBucketField {
            field: field,
            width: x.width,
            height: x.height,
            hits: hits,
        }
    }
}

fn apply_extension<P: AsRef<Path>>(path: P, extension: &str) -> PathBuf {
    let mut path_buf = path.as_ref().to_owned();
    path_buf.set_extension(extension);
    path_buf
}

use crossbeam;
use num::{Float, FromPrimitive};
use num_cpus;
use rand::distributions::range::SampleRange;
use std::cmp;
use std::io;
use std::marker::Sync;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use bucket_field::{AtomicBucketField, NonatomicBucketField};
use path_iterator;
use random_complex_generator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Buddhabrot<T> {
    width: usize,
    height: usize,
    xfocus: T, // default: -0.7
    yfocus: T, // default: 0.0
    scale: T, // factor to get from `math units` to pixels
    point_count: u64, // number of initial points to iterate
    max_iters_per_point: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Setup<T> {
    width: usize,
    height: usize,
    xfocus: T,
    yfocus: T,
    scale: Option<T>, // factor to get from `math units` to pixels
    point_count: u64, // number of initial points to iterate
    max_iters_per_point: Option<u64>,
}

impl<T> Buddhabrot<T>
where
    T: 'static + Float + FromPrimitive + SampleRange + Sync,
{
    pub fn compute(&self) -> NonatomicBucketField {
        let cpu_count = num_cpus::get() as u64;
        let mut handles = Vec::with_capacity(cpu_count as usize);
        let field = AtomicBucketField::new(self.width, self.height);
        let points_done = AtomicU64::new(0);

        crossbeam::scope(
            |scope| {
                let field = &field;
                let points_done = &points_done;
                for _ in 0..cpu_count {
                    handles.push(
                        scope.spawn(
                            move || {
                                let mut cgen = random_complex_generator::make();
                                loop {
                                    if points_done.fetch_add(1, Ordering::Relaxed) >= self.point_count {
                                        break;
                                    }

                                    for mut c in path_iterator::iterate(cgen(), self.max_iters_per_point) {
                                        c.re = c.re - self.xfocus +
                                               (T::from_usize(self.width).unwrap() /
                                                (self.scale + self.scale));
                                        c.im = c.im + self.yfocus +
                                               (T::from_usize(self.height).unwrap() /
                                                (self.scale + self.scale));
                                        c = c.scale(self.scale);
                                        field.hit(
                                            c.re.to_usize().unwrap(),
                                            c.im.to_usize().unwrap(),
                                        );
                                    }
                                }
                            },
                        ),
                    );
                }
            },
        );

        while let Some(handle) = handles.pop() {
            handle.join();
        }

        field.into()
    }
}

impl<T> Setup<T>
where
    T: 'static + Float + FromPrimitive + SampleRange + Sync,
{
    // TODO: enforce `width` and `height` bounds

    /// Sets the rendering dimensions to `width` and `height`, as well as `point_count`, the number
    /// of points to iterate.
    ///
    /// `width` and `height` must be greater than `0`.
    pub fn new(width: usize, height: usize, point_count: u64) -> Self {
        Setup {
            width: width,
            height: height,
            xfocus: T::from_f64(-0.7).unwrap(),
            yfocus: T::from_f64(0.0).unwrap(),
            scale: None,
            point_count: point_count,
            max_iters_per_point: None,
        }
    }

    /// Sets the scale to render at.
    ///
    /// The scale is a multiplier that maps units from the Buddhabrot's coordinate system onto
    /// pixels.
    ///
    /// If the scale isn't set with this function, a default scale is used.
    pub fn scale(&mut self, scale: T) {
        self.scale = Some(scale);
    }

    /// Sets the maximum number of iterations each starting point can undergo before abandoning
    /// that point.
    ///
    /// A value of `None` means never abandon prematurely; the point will iterate until it either
    /// escapes, or it is certain it will never escape.
    ///
    /// The default is `None`.
    pub fn max_iters_per_point(&mut self, max_iters_per_point: Option<u64>) {
        self.max_iters_per_point = max_iters_per_point;
    }

    /// Computes the Buddhabrot using the chosen settings, and saves the result to a png file at
    /// `path`.
    pub fn save_to_png<P: AsRef<Path>>(self, path: P) -> io::Result<()> {
        self.build().compute().save_png(path)
    }

    fn build(self) -> Buddhabrot<T> {
        let scale = self.scale
            .unwrap_or_else(
                || {
                    T::from_usize(cmp::min(self.width, self.height)).unwrap() /
                    T::from_f64(2.6).unwrap()
                },
            );
        Buddhabrot {
            width: self.width,
            height: self.height,
            xfocus: self.xfocus,
            yfocus: self.yfocus,
            scale: scale,
            point_count: self.point_count,
            max_iters_per_point: self.max_iters_per_point,
        }
    }
}

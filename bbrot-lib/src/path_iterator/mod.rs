mod cycle_detector;

use num::{Complex, Float, FromPrimitive, Zero};
use std::convert::TryFrom;

use self::cycle_detector::CycleDetector;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct PathIterator<T> {
    z: Complex<T>,
    c: Complex<T>,
    remaining: u64,
}

impl<T: Float> PathIterator<T> {
    fn new(c: Complex<T>, size: u64) -> Self {
        PathIterator {
            z: Complex::zero(),
            c: c,
            remaining: size,
        }
    }
}

impl<T: Float> Iterator for PathIterator<T> {
    type Item = Complex<T>;

    fn next(&mut self) -> Option<Complex<T>> {
        if self.remaining == 0 {
            None
        } else {
            self.z = self.z * self.z + self.c;
            self.remaining -= 1;
            Some(self.z)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match usize::try_from(self.remaining) {
            Ok(x) => (x, Some(x)),
            Err(_) => (usize::max_value(), None),
        }
    }
}

pub fn iterate<T: Float + FromPrimitive>(c: Complex<T>, max_iterations: Option<u64>) -> PathIterator<T> {
    if cardioid_check(c) || bulb_check(c) {
        return PathIterator::new(c, 0);
    }

    let mut z = Complex::zero();
    let mut cycle_detector = CycleDetector::new();
    for i in 0..max_iterations.unwrap_or(u64::max_value()) {
        if z.norm_sqr() > FromPrimitive::from_f64(4.0).unwrap() {
            return PathIterator::new(c, i);
        }
        if cycle_detector.check(z) {
            return PathIterator::new(c, 0);
        }
        z = z * z + c;
    }
    
	PathIterator::new(c, 0)
}

// Returns `true` if `c` lies within the cardioid of the Mandelbrot set.
fn cardioid_check<T: Float + FromPrimitive>(c: Complex<T>) -> bool {
    let one_quarter = FromPrimitive::from_f64(0.25).unwrap();
    let q = (c.re - one_quarter).powi(2) + c.im.powi(2);
    (q * (q + c.re - one_quarter) < one_quarter * c.im.powi(2))
}

// Returns `true` if `c` lies within the period-2 bulb of the Mandelbrot set
fn bulb_check<T: Float + FromPrimitive>(c: Complex<T>) -> bool {
    ((c.re + T::one()).powi(2) + c.im.powi(2) < FromPrimitive::from_f64(1.0 / 16.0).unwrap())
}

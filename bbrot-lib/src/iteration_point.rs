// TODO: remove duplicate code

use num::{Complex, Float, FromPrimitive, Zero};

pub fn iterate<T: Float + FromPrimitive>(c: Complex<T>) -> PathIterator<T> {
    if cardioid_check(c) || bulb_check(c) {
        return PathIterator::new(c, 0);
    }

    let mut z = Complex::zero();
    let mut cycle_detector = CycleDetector::new();
    for i in 0.. {
        if z.norm_sqr() > FromPrimitive::from_f64(4.0).unwrap() {
            return PathIterator::new(c, i);
        }
        if cycle_detector.check(z) {
            return PathIterator::new(c, 0);
        }
        z = z * z + c;
    }
    unreachable!()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct PathIterator<T> {
    z: Complex<T>,
    c: Complex<T>,
    size_hint: (usize, Option<usize>),
}

impl<T: Float> PathIterator<T> {
    fn new(c: Complex<T>, size: usize) -> Self {
        PathIterator {
            z: Complex::zero(),
            c: c,
            size_hint: (size, Some(size)),
        }
    }
}

impl<T: Float> Iterator for PathIterator<T> {
    type Item = Complex<T>;

    fn next(&mut self) -> Option<Complex<T>> {
        if self.size_hint.0 == 0 {
            None
        } else {
            self.z = self.z * self.z + self.c;
            let new_size = self.size_hint.0 - 1;
            self.size_hint = (new_size, Some(new_size));
            Some(self.z)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }
}

impl<T: Float> ExactSizeIterator for PathIterator<T> {}

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

// Cycle detection using Brent's algorithm, a.k.a teleporting turtle.
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct CycleDetector<T: Float> {
    power: usize,
    lambda: usize,
    turtle: Complex<T>,
    hare: Complex<T>,
}

impl<T: Float> CycleDetector<T> {
    fn new() -> Self {
        CycleDetector {
            power: 32,
            lambda: 0,
            turtle: Complex::new(T::nan(), T::nan()),
            hare: Complex::new(T::nan(), T::nan()),
        }
    }

    // Returns 'true' if a cycle has been found in the sequence of `next_value`s.
    fn check(&mut self, next_value: Complex<T>) -> bool {
        if self.lambda == self.power {
            self.turtle = self.hare;
            self.power *= 2;
            self.lambda = 0;
        }

        self.hare = next_value;
        self.lambda += 1;
        self.turtle == self.hare
    }
}

impl<T: Float> Default for CycleDetector<T> {
    fn default() -> Self {
        CycleDetector::new()
    }
}

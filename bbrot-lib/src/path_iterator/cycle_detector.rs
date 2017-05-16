// Cycle detection using Brent's algorithm, a.k.a teleporting turtle.

use num::{Complex, Float};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct CycleDetector<T: Float> {
    power: usize,
    lambda: usize,
    turtle: Complex<T>,
    hare: Complex<T>,
}

impl<T: Float> CycleDetector<T> {
    pub fn new() -> Self {
        CycleDetector {
            power: 32,
            lambda: 0,
            turtle: Complex::new(T::nan(), T::nan()),
            hare: Complex::new(T::nan(), T::nan()),
        }
    }

    // Returns 'true' if a cycle has been found in the sequence of `next_value`s.
    pub fn check(&mut self, next_value: Complex<T>) -> bool {
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

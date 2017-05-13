use num::{Complex, Float, FromPrimitive};
use rand;
use rand::distributions::IndependentSample;
use rand::distributions::range::SampleRange;

pub fn make<T>() -> impl FnMut() -> Complex<T>
    where T: 'static + Float + FromPrimitive + SampleRange
{
    let mut rng = rand::weak_rng();
    let range = rand::distributions::Range::new(T::from_f64(-2.0).unwrap(),
                                                T::from_f64(2.0).unwrap());
    move || Complex::new(range.ind_sample(&mut rng), range.ind_sample(&mut rng))
}

use noise::{NoiseFn, OpenSimplex};

pub fn generate() -> f64{
    let opensimplex = OpenSimplex::new(2);
    let val = opensimplex.get([1.0, 1.0]);
    return val;
}

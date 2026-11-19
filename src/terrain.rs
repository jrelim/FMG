use noise::{NoiseFn, OpenSimplex};

pub fn generate(x: f64, y: f64) -> f64{
    // we need to feed coordinates
    let opensimplex = OpenSimplex::new(2);
    let val = opensimplex.get([x, y]);
    return val;
}

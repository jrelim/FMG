use noise::{NoiseFn, OpenSimplex};

pub fn generate(noise: &OpenSimplex, x: f64, y: f64) -> f64{
    const SCALE: f64 = 0.005;
    let val = noise.get([x * SCALE, y * SCALE]);
    return val;
}

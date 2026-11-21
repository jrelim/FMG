use noise::{NoiseFn, OpenSimplex};

// constants
const SCALE: f64 = 0.003;           // base frequency
const OCTAVES: u8 = 6;              // # layers
const LACUNARITY: f64 = 2.0;        // frequency mult per layer
const PERSISTENCE: f64 = 0.5;       // amplitude mult per layer

pub fn generate(noise: &OpenSimplex, x: f64, y: f64) -> f64{
    // now we add octaves and other cool details
    let mut total = 0.0;
    let mut frequency = SCALE;
    let mut amplitude = 1.0;
    let mut max_amplitude = 0.0;

    for _ in 0..OCTAVES {
        total += amplitude * noise.get([x * frequency, y * frequency]);
        max_amplitude += amplitude;

        frequency *= LACUNARITY;
        amplitude *= PERSISTENCE;
    }
    // let val = noise.get([x * SCALE, y * SCALE]);
    // return val;
    return total / max_amplitude;
}

mod terrain;
mod render;

use noise::OpenSimplex;

fn main() {
    // do it here to save steps in terrain
    
    let seed: u32 = match std::env::args().nth(1) {
        Some(arg) => arg.parse().expect("seed should be u32"),
        None => rand::random(),
    };
    println!("Seed: {seed}");

    // throw that mug in OS
    let noise = OpenSimplex::new(seed);

    // we can probably use arrays for fixed-size images
    let mut v: Vec<Vec<f64>> = Vec::new();

    // force i32 -> f64 for opensimplex conversion
    // 2 million pixels 
    for x in 0..1080 {
        let mut row: Vec<f64> = Vec::new();
        for y in 0..1920{
            // we have to use a row vec and add to that because the vec has
            // length 0 when you instantiate it!
            row.push(terrain::generate(&noise, x as f64, y as f64));
        }
        v.push(row);
    }

    render::render(&v, &format!("images/noise_{seed}_grayscale.png"));
    render::render_color(&v, &format!("images/noise_{seed}_color.png"));

}

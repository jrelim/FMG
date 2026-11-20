mod terrain;
mod render;

use noise::OpenSimplex;

/*
fn print_vector(v: Vec<Vec<f64>>)  {
    for i in 0..v.len() {
        for j in 0..v[i].len(){
            let k: f64 = v[i][j];
            let s = format!("{:.2}", k);
            print!("{s}\t");
        }
        println!("");
    }
}
*/

fn main() {
    // do it here to save steps in terrain
    let noise = OpenSimplex::new(2);

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

    render::render(&v, "images/noise.png");

}

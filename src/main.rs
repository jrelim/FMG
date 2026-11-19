mod terrain;

fn main() {
    println!("Hello, world!");

    // we can probably use arrays for fixed-size images
    let mut v: Vec<Vec<f64>> = Vec::new();

    //let sample = terrain::generate();
    //println!("Let's see what this outputs: {sample}");
    
    // force i32 -> f64 for opensimplex conversion
    for x in 0..10 {
        let mut row: Vec<f64> = Vec::new();
        for y in 0..10{
            //v[x].push(terrain::generate(x as f64, y as f64));
            // we have to use a row vec and add to that because the vec technically has
            // length 0 when you instantiate it!
            row.push(terrain::generate(x as f64, y as f64));
        }
        v.push(row);
    }
    for i in 0..v.len() {
        for j in 0..v[i].len(){
            let k: f64 = v[i][j];
            let s = format!("{:.2}", k);
            print!("{s}\t");
        }
        println!("");
    }


}

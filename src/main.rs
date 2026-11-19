mod terrain;

fn main() {
    println!("Hello, world!");
    let mut v: Vec<f64> = Vec::new();

    //let sample = terrain::generate();
    //println!("Let's see what this outputs: {sample}");
    
    for _i in 1..10 {
        // should give us 10 'unique' f64 values
        v.push(terrain::generate());
    }
    let mut count: i32 = 0;
    for element in &v {
        println!("Index: {count}\t Data: {element}");
        count += 1;
    }


}

use image::{GrayImage, Luma};

pub fn render(buffer: &[Vec<f64>], path: &str){
    let (width, height) = dimensions(buffer);
    let (min, max) = value_bounds(buffer);
    let range = (max-min).max(f64::EPSILON); // prevent div by 0 with tiny EPSILON
    
    // what does this crap do?
    // idk let's do it how I want to
    /*
    let img = GrayImage::from_fn(width, height, |x, y| {
            let t = (buffer[y as usize][x as usize] - min) / range;
            Luma([(t * 255.0).round() as u8])
        });
    */

    let mut image = GrayImage::new(height, width);
    // my brother in christ, first run had the dims swapped :sob:
    for i in 0..height {
        for j in 0..width {
            let raw = buffer[j as usize][i as usize];
            let normalized = ((raw - min) / range).clamp(0.0, 1.0);
            let luminance = (normalized * 255.0).round() as u8;
            image.put_pixel(i, j, Luma([luminance]));
        }
    }

    let _ = image.save(path);
}

fn dimensions(buffer: &[Vec<f64>]) -> (u32, u32){
    // currently assumes well-behaving grid
    let width: u32 = buffer.len() as u32;
    let height: u32 = buffer[0].len() as u32;
    
    return (width, height);
}

fn value_bounds(buffer: &[Vec<f64>]) -> (f64, f64) {
    let mut min: f64 = f64::INFINITY;
    let mut max: f64 = f64::NEG_INFINITY;

    // now we actually write code that works
    for &x in buffer.iter().flatten() {
        if x < min{
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    return (min, max);
}

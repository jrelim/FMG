use image::{GrayImage, Luma, Rgb, RgbImage};

pub fn render(buffer: &[Vec<f64>], path: &str){
    let (width, height) = dimensions(buffer);
    let (min, max) = value_bounds(buffer);
    let range = (max-min).max(f64::EPSILON); // prevent div by 0 with tiny EPSILON
    
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

pub fn render_color(buffer: &[Vec<f64>], path: &str){
    let (width, height) = dimensions(buffer);
    // let (min, max) = value_bounds(buffer);
    // let range = (max - min).max(f64::EPSILON);

    let mut image = RgbImage::new(height, width);
    
    for i in 0..height {
        for j in 0..width {
            let raw = buffer[j as usize][i as usize];
            let h = ((raw + 1.0) / 2.0).clamp(0.0, 1.0);
            image.put_pixel(i, j, color_for(h));
        }
    }

    if let Err(e) = image.save(path) {
        eprint!("couldn't save {path}: {e}");
    }
    
}

fn color_for(h: f64) -> Rgb<u8> {
    if h < 0.35 {
        Rgb([20, 40, 110])       // deep water
    } else if h < 0.45 {
        Rgb([40, 80, 160])       // shallow water
    } else if h < 0.48 {
        Rgb([210, 200, 140])     // beach
    } else if h < 0.62 {
        Rgb([70, 140, 60])       // grass
    } else if h < 0.75 {
        Rgb([35, 90, 40])        // forest
    } else if h < 0.85 {
        Rgb([110, 100, 90])      // rock
    } else {
        Rgb([240, 240, 245])     // snow
    }
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

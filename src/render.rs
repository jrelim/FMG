use image::{GrayImage, Luma, Rgb, RgbImage};

// direction *toward* the sun in image coords (x right, y DOWN, z up).
// (-1, -1.5) = up-left on the image. z sets elevation: tan(elev) = z / |(x, y)|
const LIGHT: [f64; 3] = [-1.0, -1.5, 0.6];
// heights are in [-1, 1] over ~2000px, so stretch them or nothing has slope
// cuz the slope is ridiculously small. like the sun over a fish tank of sand
const Z_EXAGGERATION: f64 = 150.0;  // higher = more and longer shadows
// raw height of the shoreline (h = 0.45 in color_for => raw = -0.1)
const SEA_LEVEL: f64 = -0.1;
// brightness of fully unlit ground (0 = black)
const AMBIENT: f64 = 0.35;
// how much direct light still leaks into cast shadows (0 = none)
const SHADOW_LEAK: f64 = 0.25;

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

pub fn render_shaded(buffer: &[Vec<f64>], path: &str) {
    let (rows, cols) = dimensions(buffer);
    let (rows, cols) = (rows as usize, cols as usize);
 
    // the heights everything below agrees on. water flattened to sea level,
    // then stretched vertically. shading and shadows MUST use the same ones
    let mut z: Vec<Vec<f64>> = Vec::with_capacity(buffer.len());
    
    for row in buffer.iter() {
        let mut new_row: Vec<f64> = Vec::with_capacity(row.len());

        for &v in row.iter() {
            // clamp, anything < sea level now exactly sea level
            let clamped = v.max(SEA_LEVEL);
            // stretch into "pixel-sized" height units
            let stretched = clamped * Z_EXAGGERATION;
            new_row.push(stretched);
        }

        z.push(new_row);
    }
 
    let light = normalize(LIGHT);
    let horiz = LIGHT[0].hypot(LIGHT[1]);
    let tan_elev = LIGHT[2] / horiz; // ray rise per pixel of horizontal travel
    let max_h = value_bounds(&z).1;
    let diagonal = (rows as f64).hypot(cols as f64);
    let offsets = shadow_offsets((LIGHT[0], LIGHT[1]), diagonal);
 
    // cast shadows, march toward the sun from every pixel
    let mut shadow = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let h_p = z[r][c];
            for &(ox, oy, t) in &offsets {
                // elev of ray
                let ray = h_p + t * tan_elev;
                if ray > max_h {
                    break; // ray is above everything, nothing can block it
                }
                let qr = r as i32 + oy;
                let qc = c as i32 + ox;
                if qr < 0 || qc < 0 || qr >= rows as i32 || qc >= cols as i32 {
                    break; // walked off the map
                }
                if z[qr as usize][qc as usize] > ray {
                    shadow[r][c] = true;
                    break;
                }
            }
        }
    }

    // slope shading (Lambert) + combine
    let mut image = RgbImage::new(cols as u32, rows as u32);
    for r in 0..rows {
        for c in 0..cols {
            // central differences, clamped at the edges
            let (r0, r1) = (r.saturating_sub(1), (r + 1).min(rows - 1));
            let (c0, c1) = (c.saturating_sub(1), (c + 1).min(cols - 1));
            let dzdx = (z[r][c1] - z[r][c0]) / (c1 - c0) as f64;
            let dzdy = (z[r1][c] - z[r0][c]) / (r1 - r0) as f64;

            let normal = normalize([-dzdx, -dzdy, 1.0]);
            // divide by light.z so flat ground = 1.0 and the palette stays true
            let mut lit = (dot(normal, light).max(0.0) / light[2]).min(1.6);
            if shadow[r][c] {
                lit *= SHADOW_LEAK;
            }
            let factor = AMBIENT + (1.0 - AMBIENT) * lit;

            let h = ((buffer[r][c] + 1.0) / 2.0).clamp(0.0, 1.0);
            image.put_pixel(c as u32, r as u32, shade(color_for(h), factor));
        }
    }

    if let Err(e) = image.save(path) {
        eprintln!("couldn't save {path}: {e}");
    }
}
 
fn shade(color: Rgb<u8>, factor: f64) -> Rgb<u8> {
    let [r, g, b] = color.0;
    // inline func
    let f = |v: u8| (v as f64 * factor).round().clamp(0.0, 255.0) as u8;
    Rgb([f(r), f(g), f(b)])
}
 
fn normalize(v: [f64; 3]) -> [f64; 3] {
    // standard normalization of vec
    let len = dot(v, v).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}
 
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    // standard dot product
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
 
// bresenham step offsets (ox, oy, t) marching from (0, 0)
// toward sun_dir up to max_len pixels.
fn shadow_offsets(sun_dir: (f64, f64), max_len: f64) -> Vec<(i32, i32, f64)> {
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm#Algorithm
    let len = sun_dir.0.hypot(sun_dir.1);
    if len == 0.0 || max_len <= 0.0 {
        return Vec::new();
    }

    // normalize and stretch to max_len rounded to nearest whole #
    let x1 = (sun_dir.0 / len * max_len).round() as i32;
    let y1 = (sun_dir.1 / len * max_len).round() as i32;

    let dx = x1.abs();
    let dy = -y1.abs();
    let sx = x1.signum(); // returns -1, 0, or 1
    let sy = y1.signum();
    let mut err = dx + dy;

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut offsets = Vec::with_capacity(dx.max(y1.abs()) as usize);

    loop {
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
        // diag dist traveled
        let t = (x as f64).hypot(y as f64);
        offsets.push((x, y, t));
    }

    offsets
}

fn color_for(h: f64) -> Rgb<u8> {
    // edit to account for shading
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

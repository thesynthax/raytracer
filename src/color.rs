use crate::vec3::Color;
use raytracer::clamp;

pub fn write_color(pixel_color: Color, samples: i32)
{
    let mut r: f32 = pixel_color.r();
    let mut g: f32 = pixel_color.g();
    let mut b: f32 = pixel_color.b();
    
    let scale: f32 = 1.0 / samples as f32;
    r = (r*scale).sqrt();
    g = (g*scale).sqrt();
    b = (b*scale).sqrt();

    let ir = (256 as f32 * clamp(r, 0.0, 0.9999)) as i32;
    let ig = (256 as f32 * clamp(g, 0.0, 0.9999)) as i32;
    let ib = (256 as f32 * clamp(b, 0.0, 0.9999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

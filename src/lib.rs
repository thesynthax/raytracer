use std::f32::consts::PI;

pub fn clamp(x: f32, min: f32, max: f32) -> f32
{
    if x > max { return max; }
    if x < min { return min; }
    x
}

pub fn min(a: f32, b: f32) -> f32
{
    if a < b { a } else { b }
}

pub fn deg_to_rad(deg: f32) -> f32
{
    deg * PI / 180.0
}

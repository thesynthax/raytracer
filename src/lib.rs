pub fn clamp(x: f32, min: f32, max: f32) -> f32
{
    if x > max { return max; }
    if x < min { return min; }
    x
}

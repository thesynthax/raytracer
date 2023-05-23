use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera
{
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera
{
    pub fn camera() -> Self //Camera
    {
        const ASPECT_RATIO: f32 = 16.0/9.0;
        const VIEWPORT_HEIGHT: f32 = 2.0;
        const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH); 
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        Ray::ray(self.origin, (self.lower_left_corner + self.horizontal*u + self.vertical*v) - self.origin)
    }
}

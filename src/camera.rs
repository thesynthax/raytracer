use crate::vec3::*;
use crate::ray::Ray;
use raytracer::*;

pub struct Camera
{
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera
{
    pub fn camera(fov: f32, aspect_ratio: f32, look_from: Point, look_at: Point, vup: Vec3) -> Self //Camera
    {
        let theta: f32 = deg_to_rad(fov);
        let h: f32 = (theta/2.0).tan();
        let VIEWPORT_HEIGHT: f32 = 2.0 * h;
        let VIEWPORT_WIDTH: f32 = aspect_ratio * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = u * VIEWPORT_WIDTH;
        let vertical = v * VIEWPORT_HEIGHT;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w; 
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        Ray::ray(self.origin, (self.lower_left_corner + self.horizontal*u + self.vertical*v) - self.origin)
    }
}

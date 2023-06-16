use crate::{vec3::*, ray::Ray};
use raytracer::*;

#[derive(Default, Clone, Copy)]
pub struct AABB
{
    pub min: Point,
    pub max: Point
}

impl AABB
{
    pub fn aabb(min: Point, max: Point) -> AABB
    {
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool
    {
        for i in 0..3
        {
            let t0: f32 = min((self.min.this()[i] - r.origin().this()[i]) / r.direction().this()[i], (self.max.this()[i] - r.origin().this()[i]) / r.direction().this()[i]);    
            let t1: f32 = max((self.min.this()[i] - r.origin().this()[i]) / r.direction().this()[i], (self.max.this()[i] - r.origin().this()[i]) / r.direction().this()[i]);    

            t_min = max(t0, t_min);
            t_max = min(t1, t_max);

            if t_max <= t_min
            {
                return false;        
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB
    {
        let small: Point = Point::new(min(box0.min.x(), box1.min.x()),
                                      min(box0.min.y(), box1.min.y()),
                                      min(box0.min.z(), box1.min.z()));
        let big: Point = Point::new(max(box0.max.x(), box1.max.x()),
                                      max(box0.max.y(), box1.max.y()),
                                      max(box0.max.z(), box1.max.z()));

        AABB::aabb(small, big)
    }
}

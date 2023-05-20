use crate::hittable::*;
use crate::vec3::{Vec3, Point};
use crate::ray::Ray;

pub struct Sphere
{
    center: Point,
    radius: f32
} 

impl Sphere
{
    pub fn sphere(center: Point, radius: f32) -> Sphere
    {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_info: &mut HitInfo) -> bool
    {
        /*
            * if (P-C).(P-C)=r^2 where P=A+tB vector and C=center
            * so expanding,
            * t^2(B.B) + 2tB.(A-C) + (A-C).(A-C) - r^2 = 0
            * so if D > 0, then the ray has hit the sphere    
        */
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = Vec3::dot(&r.direction(), &r.direction());
        let b: f32 = 2.0 * Vec3::dot(&r.direction(), &oc);
        let c: f32 = Vec3::dot(&oc, &oc) - self.radius*self.radius;

        let disc = b*b - 4.0*a*c;
        
        if disc < 0.0
        {
            false
        }
        else 
        {    
            let sqrtd: f32 = disc.sqrt();
            //t_min < t < t_max where t is the root
            let mut t: f32 = (-b - sqrtd)/(2.0*a);

            if (t > t_max || t < t_min)
            {
                t = (-b + sqrtd)/(2.0*a);
                if (t > t_max || t < t_min)
                {
                    false;
                }
            }

            hit_info.set_t(t);
            hit_info.set_p(r.parametric_point(t));
            let outward_normal: Vec3 = (hit_info.p() - self.center)/self.radius;
            hit_info.set_front_normal(r, &outward_normal);

            true
        }
    }
}

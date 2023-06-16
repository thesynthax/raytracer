use crate::hittable::*;
use crate::material::*;
use crate::vec3::{Vec3, Point};
use crate::ray::Ray;
use crate::aabb::*;

pub struct XYRect
{
    x0: f32, x1: f32, y0: f32, y1: f32, k: f32,
    mat: Material
}

pub struct YZRect
{
    y0: f32, y1: f32, z0: f32, z1: f32, k: f32,
    mat: Material
}

pub struct ZXRect
{
    z0: f32, z1: f32, x0: f32, x1: f32, k: f32,
    mat: Material
}

impl XYRect
{
    pub fn xyrect(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Material) -> XYRect
    {
        XYRect { x0, x1, y0, y1, k, mat }
    }
}

impl YZRect
{
    pub fn yzrect(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, mat: Material) -> YZRect
    {
        YZRect { y0, y1, z0, z1, k, mat }
    }
}

impl ZXRect
{
    pub fn zxrect(z0: f32, z1: f32, x0: f32, x1: f32, k: f32, mat: Material) -> ZXRect
    {
        ZXRect { z0, z1, x0, x1, k, mat }
    }
}

impl Hittable for XYRect
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>
    {
        let t: f32 = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max
        {
            return None;
        }

        let x: f32 = r.origin().x() + r.direction().x() * t;
        let y: f32 = r.origin().y() + r.direction().y() * t;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1
        {
            return None;
        }

        return Some(HitInfo { t, p: r.parametric_point(t), normal: Vec3::new(0.0, 0.0, 1.0), mat: self.mat });
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool 
    {
        *output_box = AABB::aabb(Point::new(self.x0, self.y0, self.k-0.0001), Point::new(self.x1, self.y1, self.k+0.0001));
        return true;
    }
}

impl Hittable for YZRect
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>
    {
        let t: f32 = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max
        {
            return None;
        }

        let y: f32 = r.origin().y() + r.direction().y() * t;
        let z: f32 = r.origin().z() + r.direction().z() * t;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1
        {
            return None;
        }

        return Some(HitInfo { t, p: r.parametric_point(t), normal: Vec3::new(1.0, 0.0, 0.0), mat: self.mat });
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool 
    {
        *output_box = AABB::aabb(Point::new(self.k-0.0001, self.y0, self.z0), Point::new(self.k+0.0001, self.y1, self.z1));
        return true;
    }
}

impl Hittable for ZXRect
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>
    {
        let t: f32 = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max
        {
            return None;
        }

        let z: f32 = r.origin().z() + r.direction().z() * t;
        let x: f32 = r.origin().x() + r.direction().x() * t;
        if z < self.z0 || z > self.z1 || x < self.x0 || x > self.x1
        {
            return None;
        }

        return Some(HitInfo { t, p: r.parametric_point(t), normal: Vec3::new(0.0, 1.0, 0.0), mat: self.mat });
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool 
    {
        *output_box = AABB::aabb(Point::new(self.x0, self.k-0.0001, self.z0), Point::new(self.x1, self.k+0.0001, self.z1));
        return true;
    }
}

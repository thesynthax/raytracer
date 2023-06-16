use crate::hittable::*;
use crate::material::*;
use crate::vec3::{Vec3, Point};
use crate::hittable_list::*;
use crate::aarect::*;
use crate::aabb::*;

pub struct Cuboid
{
    min: Point,
    max: Point,
    sides: HittableList
}

impl Cuboid
{
    pub fn cuboid(p0: Point, p1: Point, mat: Material) -> Cuboid
    {
        let mut sides: Vec<Box<dyn Hittable>> = Vec::new();

        sides.push(Box::new(XYRect::xyrect(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat)));
        sides.push(Box::new(XYRect::xyrect(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat)));
        sides.push(Box::new(YZRect::yzrect(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat)));
        sides.push(Box::new(YZRect::yzrect(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat)));
        sides.push(Box::new(ZXRect::zxrect(p0.z(), p1.z(), p0.x(), p1.x(), p1.y(), mat)));
        sides.push(Box::new(ZXRect::zxrect(p0.z(), p1.z(), p0.x(), p1.x(), p0.y(), mat)));

        let list: HittableList = HittableList::new(sides);

        Cuboid { min: p0, max: p1, sides: list }
    }
}

impl Hittable for Cuboid
{
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitInfo> 
    {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool 
    {
        *output_box = AABB::aabb(self.min, self.max);
        return true;
    }
}

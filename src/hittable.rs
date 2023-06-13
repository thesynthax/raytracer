use crate::vec3::{Vec3, Point, Color};
use crate::ray::Ray;
use crate::material::Material;

pub trait Hittable 
{
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitInfo>;
}

//#[derive(Default)]
pub struct HitInfo 
{
    pub t: f32,
    pub p: Point,
    pub normal: Vec3,
    //front_face: bool,
    pub mat: Material
}

/*impl Default for HitInfo
{
    fn default() -> Self 
    {
        HitInfo { t: 0.0, p: Point::zero(), normal: Vec3::zero(), front_face: false, mat: Material::Lambertian { albedo: Color::zero() } }    
    }
}*/

impl HitInfo 
{
    /*pub fn p(&self) -> Point 
    {
        self.p
    }
    pub fn t(&self) -> f32 
    {
        self.t
    }
    pub fn normal(&self) -> Vec3 
    {
        self.normal
    }
    pub fn front_face(&self) -> bool
    {
        self.front_face
    }
    pub fn mat(&self) -> Material
    {
        self.mat
    }
    pub fn set_p(&mut self, val: Point)
    {
        self.p = val
    }
    pub fn set_t(&mut self, val: f32)
    {
        self.t = val
    }
    pub fn set_normal(&mut self, val: Vec3)
    {
        self.normal = val
    }
    pub fn set_front_face(&mut self, val: bool)
    {
        self.front_face = val
    }
    
    pub fn set_material(&mut self, val: Material)
    {
        self.mat = val
    }*/
    /*pub fn set_front_normal(&mut self, r: &Ray, outward_normal: &Vec3)
    {
        self.front_face = Vec3::dot(outward_normal, &r.direction()) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }*/
}


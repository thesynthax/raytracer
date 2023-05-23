use crate::ray::Ray;
use crate::hittable::*;

pub struct HittableList
{
    hittables: Vec<Box<dyn Hittable>>
}

impl HittableList
{
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> HittableList
    {
        HittableList { hittables }
    }
}

impl Hittable for HittableList
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, hit_info: &mut HitInfo) -> bool
    {
        let mut temp: HitInfo = HitInfo::default();
        let mut hit: bool = false;
        let mut closest_hit = t_max;

        for hittable in &self.hittables
        {
            if hittable.hit(r, t_min, closest_hit, &mut temp)
            {
                hit = true;
                closest_hit = temp.t();
                hit_info.set_t(temp.t());
                hit_info.set_p(temp.p());
                hit_info.set_normal(temp.normal());
                //let outward_normal: Vec3 = (hit_info.p() - hittable.&hittable)
                //hit_info.set_front_normal(r, &temp.normal());
                //hit_info = &mut temp;
            }
        }
        hit
    }
}

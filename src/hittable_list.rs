use crate::ray::Ray;
use crate::hittable::*;
use crate::aabb::*;

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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>
    {
        let mut hitInfo: Option<HitInfo> = None;
        //let mut hit: bool = false;
        let mut closest_hit = t_max;

        for hittable in &self.hittables
        {
            /*if hittable.hit(r, t_min, closest_hit)
            {
                hit = true;
                closest_hit = temp.t;
                hit_info.t = temp.t;
                hit_info.p = temp.p;
                hit_info.normal = temp.normal;
                //let outward_normal: Vec3 = (hit_info.p() - hittable.&hittable)
                //hit_info.set_front_normal(r, &temp.normal());
                //hit_info = &mut temp;
            }*/
            if let Some(i) = hittable.hit(r, t_min, closest_hit)
            {
                closest_hit = i.t;
                hitInfo = Some(i);
            }
        }
        hitInfo
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool 
    {
        if self.hittables.is_empty()
        {
            return false;
        }

        let mut temp_box: AABB = AABB::default();
        let mut first_box: bool = true;

        for hittable in &self.hittables
        {
            if !hittable.bounding_box(&mut temp_box)
            {
                return false;
            }
            *output_box = if first_box { temp_box } else { AABB::surrounding_box(*output_box, temp_box) };
            first_box = false;
        }

        return true;
    }
}

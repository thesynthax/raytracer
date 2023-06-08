use crate::vec3::*;
use crate::ray::Ray;
use crate::hittable::HitInfo;

#[derive(Clone, Copy)]
pub enum Material
{
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 }
}

/*impl Default for Material
{
    fn default() -> Self 
    {
        Material::Lambertian { albedo: Color::one() }
    }
}*/

pub fn scatter(mat: &Material, ray_in: &Ray, hit_info: &HitInfo, att: &mut Color, scattered: &mut Ray) -> bool
{
    match mat
    {
        &Material::Lambertian { albedo } => 
        {
            /*let mut scatter_direction = hit_info.normal() + Vec3::random_unit_vector();

            if scatter_direction.near_zero()
            {
                scatter_direction = hit_info.normal();
            }

            *scattered = Ray::ray(hit_info.p(), scatter_direction);*/
            let target = hit_info.p + hit_info.normal + Vec3::random_in_unit_sphere();        
            *scattered = Ray::ray(hit_info.p, target - hit_info.p);
            *att = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } =>
        {
            let mut f = 1.0;
            return false;
        }
    }
}

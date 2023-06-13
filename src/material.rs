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
            let mut scatter_direction = hit_info.normal + Vec3::random_in_unit_sphere();

            if scatter_direction.near_zero()
            {
                scatter_direction = hit_info.normal;
            }

            *scattered = Ray::ray(hit_info.p, scatter_direction);
            //let target = hit_info.p + hit_info.normal + Vec3::random_in_unit_sphere();        
            //let target = hit_info.p + Vec3::random_in_hemisphere(hit_info.normal);
            //let target = hit_info.normal + Vec3::random_unit_vector();
            //*scattered = Ray::ray(hit_info.p, target - hit_info.p);
            *att = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } =>
        {
            let mut f = 0.0;
            let reflected: Vec3 = Vec3::reflect(Vec3::unit_vector(&ray_in.direction()), hit_info.normal);
            *scattered = Ray::ray(hit_info.p, reflected + Vec3::random_in_unit_sphere() * fuzz);
            *att = albedo;
            return Vec3::dot(&scattered.direction(), &hit_info.normal) > 0.0;
        }
    }
}

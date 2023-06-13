use crate::vec3::*;
use crate::ray::Ray;
use crate::hittable::HitInfo;
use rand::prelude::*;
use raytracer::*;

#[derive(Clone, Copy)]
pub enum Material
{
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { ref_index: f32 }
}

/*impl Default for Material
{
    fn default() -> Self 
    {
        Material::Lambertian { albedo: Color::one() }
    }
}*/

fn schlick(cosine: f32, ref_index: f32) -> f32 
{
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

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
        &Material::Dielectric { ref_index } =>
        {
            /*let mut outward_normal = Vec3::default();
            let reflected = Vec3::reflect(ray_in.direction(), hit_info.normal);
            let mut ni_over_nt = 0.0;
            *att = Vec3::new(1.0, 1.0, 1.0);

            let mut refracted = Vec3::default();

            let mut reflect_prob = 0.0;
            let mut cosine = 0.0;

            if Vec3::dot(&ray_in.direction(), &hit_info.normal) > 0.0 {
                outward_normal = hit_info.normal;
                ni_over_nt = ref_index;
                cosine = ref_index * Vec3::dot(&ray_in.direction(), &hit_info.normal)
                    / ray_in.direction().length();
            } else {
                outward_normal = hit_info.normal;
                ni_over_nt = 1.0 / ref_index;
                cosine = -Vec3::dot(&ray_in.direction(), &hit_info.normal) / ray_in.direction().length();
            }

            if Vec3::refract(&ray_in.direction(), &outward_normal, ni_over_nt, &mut refracted) 
            {
                reflect_prob = schlick(cosine, ref_index);
            }
            else
            {
                reflect_prob = 1.0;
            }

            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob
            {
                *scattered = Ray::ray(hit_info.p, reflected);
            }
            else 
            {
                *scattered = Ray::ray(hit_info.p, refracted);
            }

            return true;*/

            let mut outward_normal = Vec3::default();
            let reflected = Vec3::reflect(ray_in.direction(), hit_info.normal);
            let mut ni_over_nt = 0.0;
            let mut reflect_prob = 0.0;

            *att = Color::one();

            if Vec3::dot(&ray_in.direction(), &hit_info.normal) < 0.0
            {
                outward_normal = hit_info.normal;
                ni_over_nt = 1.0 / ref_index;
                //cosine = ref_index * Vec3::dot(&ray_in.direction(), &hit_info.normal) / ray_in.direction().length();
            } 
            else
            {
                outward_normal = -hit_info.normal;
                ni_over_nt = ref_index;
                //cosine = -Vec3::dot(&ray_in.direction(), &hit_info.normal) / ray_in.direction().length();
            }

            let unit_direction = Vec3::unit_vector(&ray_in.direction());

            let cos_theta = min(Vec3::dot(&(-unit_direction), &hit_info.normal), 1.0);
            let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

            let can_refract = ni_over_nt * sin_theta < 1.0;

            let out_direction = if (can_refract || schlick(cos_theta, ref_index) < rand::thread_rng().gen::<f32>()) { Vec3::refract(&unit_direction, &outward_normal, ni_over_nt) } else { Vec3::reflect(unit_direction, outward_normal) };

            *scattered = Ray::ray(hit_info.p, out_direction);

            return true;
        }
    }
}

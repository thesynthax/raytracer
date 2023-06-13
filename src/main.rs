mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod material;

use vec3::{Vec3, Color, Point};
use ray::Ray;
use sphere::Sphere;
use hittable::*;
use hittable_list::HittableList;
use camera::Camera;
use material::*;
use rand::prelude::*;

use crate::color::write_color;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color
{
    //let mut hitinfo: HitInfo = HitInfo::default();

    /*if depth < 0
    {
        return Color::zero();
    }*/

    //Hittables (spheres etc)
    if let Some(i) = world.hit(r, 0.001, std::f32::MAX)    
    {
        //let target: Point = i.p + Vec3::random_in_hemisphere(i.normal);
        //return ray_color(&Ray::ray(i.p, target - i.p), world, depth - 1) / 2.0;

        let mut scattered: Ray = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation: Color = Color::default();

        if depth > 0 && scatter(&i.mat, r, &i, &mut attenuation, &mut scattered)
        {
            return ray_color(&scattered, world, depth-1) * attenuation;
        }
        else
        {
            return Color::zero(); 
        }
    }

    //Sky
    let unit_direction = Vec3::unit_vector(&(r.direction()));
    let t: f32 = (unit_direction.y() + 1.0)/2.0;

    let white = Color::one(); //Color::zero() would be black
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let blended_value: Color = white + (blue - white) * t;
    blended_value
}

fn main()
{
    //Image
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const MAX_VALUE: i32 = 255;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //Camera
    let cam: Camera = Camera::camera();

    //World
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian { albedo: Color::new(0.8, 0.3, 0.3) })));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) })));
    hittables.push(Box::new(Sphere::sphere(Point::new(1.4, 0.2, -1.4), 0.4, Material::Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.2 })));
    hittables.push(Box::new(Sphere::sphere(Point::new(-1.4, -0.1, -1.2), 0.3, Material::Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.5 })));
    let world: HittableList = HittableList::new(hittables);

    //Random Number Generator
    let mut rng = rand::thread_rng();

    //Render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);
    for j in (0..IMAGE_HEIGHT).rev()
    {
        for i in 0..IMAGE_WIDTH 
        {
            let mut pixel_color: Color = Color::zero();

            for _s in 0..SAMPLES_PER_PIXEL
            {
                let u = (i as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                let v = (j as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;

                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

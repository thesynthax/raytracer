mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;

use vec3::{Vec3, Color, Point};
use ray::Ray;
use sphere::Sphere;
use hittable::*;
use hittable_list::HittableList;
use camera::Camera;
use rand::prelude::*;

use crate::color::write_color;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color
{
    let mut hitinfo: HitInfo = HitInfo::default();

    if depth < 0
    {
        return Color::zero();
    }

    //Hittables (spheres etc)
    if world.hit(r, 0.0, std::f32::MAX, &mut hitinfo)    
    {
        let target: Point = hitinfo.p() + hitinfo.normal() + Vec3::random_unit_vector();
        return ray_color(&Ray::ray(hitinfo.p(), target - hitinfo.p()), world, depth - 1) / 2.0;
        //return (Color::new(hitinfo.normal().x()+1.0, hitinfo.normal().y()+1.0, hitinfo.normal().z()+1.0))/2.0
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
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, 0.0, -1.0), 0.5)));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, -100.5, -1.0), 100.0)));
    //hittables.push(Box::new(Sphere::sphere(Point::new(1.4, 0.2, -1.4), 0.4)));
    //hittables.push(Box::new(Sphere::sphere(Point::new(-1.4, -0.1, -1.2), 0.3)));
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

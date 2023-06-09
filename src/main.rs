mod vec3;
mod ray;
mod color;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod material;
mod aarect;
mod aabb;
mod cuboid;

use vec3::{Vec3, Color, Point};
use ray::Ray;
use sphere::Sphere;
use hittable::*;
use hittable_list::HittableList;
use camera::Camera;
use material::*;
use rand::prelude::*;
use aarect::*;
use cuboid::*;

use crate::color::write_color;

fn ray_color(r: &Ray, world: &HittableList, bg: &Color, depth: i32) -> Color
{
    if depth < 0
    {
        return Color::zero();
    }

    let mut scattered: Ray = Ray::ray(Vec3::default(), Vec3::default());
    let mut attenuation: Color = Color::default();
    let mut emitted: Color = Color::default();

    if let Some(i) = world.hit(r, 0.001, std::f32::MAX)
    {
        emitted = emit(&i.mat);
        if !scatter(&i.mat, r, &i, &mut attenuation, &mut scattered)
        {
            return emitted; 
        }
        return emitted + ray_color(&scattered, world, bg, depth-1) * attenuation;
    }

    return *bg; 
    //let mut hitinfo: HitInfo = HitInfo::default();

    /*if depth < 0
    {
        return Color::zero();
    }

    //Hittables (spheres etc)
    if let Some(i) = world.hit(r, 0.001, std::f32::MAX)    
    {
        //let target: Point = i.p + Vec3::random_in_hemisphere(i.normal);
        //return ray_color(&Ray::ray(i.p, target - i.p), world, depth - 1) / 2.0;

        let mut scattered: Ray = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation: Color = Color::default();

        if depth > 0 && scatter(&i.mat, r, &i, &mut attenuation, &mut scattered)
        {
            return ray_color(&scattered, world, bg, depth-1) * attenuation;
        }
        return Color::zero(); 
    }
    else 
    {
        return *bg;
    }*/

    /*Sky
    let unit_direction = Vec3::unit_vector(&(r.direction()));
    let t: f32 = (unit_direction.y() + 1.0)/2.0;

    let white = Color::one(); //Color::zero() would be black
    let blue = Vec3::new(0.5, 0.7, 1.0);

    let blended_value: Color = white + (blue - white) * t;
    blended_value*/
}

fn cornell_box() -> HittableList
{
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    let red = Material::Lambertian { albedo: Color::new(0.65, 0.05, 0.05) };
    let green = Material::Lambertian { albedo: Color::new(0.12, 0.45, 0.15) };
    let white = Material::Lambertian { albedo: Color::new(0.73, 0.73, 0.73) };
    let light = Material::Emissive { light_color: Color::one()*25.0 };

    hittables.push(Box::new(YZRect::yzrect(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    hittables.push(Box::new(YZRect::yzrect(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    hittables.push(Box::new(ZXRect::zxrect(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    hittables.push(Box::new(ZXRect::zxrect(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    hittables.push(Box::new(ZXRect::zxrect(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    hittables.push(Box::new(XYRect::xyrect(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    HittableList::new(hittables)
}

fn final_scene() -> HittableList
{
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    let red = Material::Lambertian { albedo: Color::new(0.65, 0.05, 0.05) };
    let green = Material::Lambertian { albedo: Color::new(0.12, 0.45, 0.15) };
    let white = Material::Lambertian { albedo: Color::new(0.73, 0.73, 0.73) };
    let light = Material::Emissive { light_color: Color::one()*7.0 };
    let metal_shiny = Material::Metal { albedo: Color::one(), fuzz: 0.0 };
    let metal_fuzzy = Material::Metal { albedo: Color::one(), fuzz: 0.6 };
    let glass = Material::Dielectric { ref_index: 1.5 };

    for i in 0..20
    {
        for j in 0..20
        {
            let width = 100.0;
            let x0 = -1000.0 + i as f32 *width;
            let z0 = -1000.0 + j as f32 *width;
            let y0 = 0.0;
            let x1 = x0 + width;
            let y1 = rand::thread_rng().gen::<f32>() * 100.0;
            let z1 = z0 + width;

            hittables.push(Box::new(Cuboid::cuboid(Point::new(x0, y0, z0), Point::new(x1, y1, z1), green)));
        }
    }

    hittables.push(Box::new(ZXRect::zxrect(123.0, 423.0, 147.0, 412.0, 554.0, light)));
    hittables.push(Box::new(Sphere::sphere(Point::new(260.0, 150.0, 45.0), 50.0, glass)));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, 150.0, 45.0), 50.0, metal_fuzzy)));
    hittables.push(Box::new(Sphere::sphere(Point::new(400.0, 200.0, 400.0), 50.0, metal_shiny)));

    HittableList::new(hittables)
}

fn main()
{
    //Image
    const ASPECT_RATIO: f32 = 1.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const MAX_VALUE: i32 = 255;
    const SAMPLES_PER_PIXEL: i32 = 1000;
    const MAX_DEPTH: i32 = 50;

    //Camera
    //let cam: Camera = Camera::camera(90.0, ASPECT_RATIO, Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0));
    let cam: Camera = Camera::camera(40.0, ASPECT_RATIO, Point::new(478.0, 278.0, -600.0), Point::new(278.0, 278.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    //World
    /*let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian { albedo: Color::new(0.8, 0.3, 0.3) })));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) })));
    hittables.push(Box::new(Sphere::sphere(Point::new(1.4, 0.2, -1.4), 0.4, Material::Metal { albedo: Color::new(0.8, 0.8, 0.8), fuzz: 0.2 })));
    hittables.push(Box::new(Sphere::sphere(Point::new(-1.0, 0.0, -1.0), -0.5, Material::Dielectric { ref_index: 1.5 })));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.1, 0.1, -0.4), 0.1, Material::Emissive { light_color: Color::new(0.0, 1.0, 1.0)*5.0 })));
    hittables.push(Box::new(XYRect::xyrect(0.0, 2.0, -1.0, 1.0, -1.7, Material::Emissive { light_color: Color::one() } )));
    hittables.push(Box::new(Cuboid::cuboid(Point::new(-1.5, 0.0, -2.0), Point::new(-0.5, 1.0, -1.0), Material::Lambertian { albedo: Color::one() })));
    let world: HittableList = HittableList::new(hittables);
    //let world = cornell_box();*/

    let world = final_scene();

    //Background
    let bg = Color::new(0.05,0.1,0.2);
    //let bg = Color::zero();

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
                pixel_color += ray_color(&r, &world, &bg, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

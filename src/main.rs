mod vec3;
mod ppm;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;

use vec3::{Vec3, Color, Point};
use ray::Ray;
use sphere::Sphere;
use hittable::*;
use hittable_list::HittableList;

fn ray_color(r: &Ray, world: &HittableList) -> Color
{
    let mut hitinfo: HitInfo = HitInfo::default();

    if world.hit(r, 0.0, std::f32::MAX, &mut hitinfo)
    {
        //return Color::zero();
        return (Color::new(hitinfo.normal().x()+1.0, hitinfo.normal().y()+1.0, hitinfo.normal().z()+1.0))/2.0
    }
    else
    {
        let unit_direction = Vec3::unit_vector(&(r.direction()));
        let t: f32 = (unit_direction.y() + 1.0)/2.0;

        let white = Color::one(); //Color::zero would be black
        let blue = Vec3::new(0.5, 0.7, 1.0);

        let blended_value: Color = white + (blue - white) * t;
        blended_value
    }
}

fn main()
{
    //Image
    const ASPECT_RATIO: f32 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const MAX_VALUE: i32 = 255;

    //Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH); 

    //World
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::new();
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, 0.0, -1.0), 0.5)));
    hittables.push(Box::new(Sphere::sphere(Point::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(hittables);


    //Render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);
    for j in (0..IMAGE_HEIGHT).rev()
    {
        for i in 0..IMAGE_WIDTH 
        {
            let u = i as f32 / IMAGE_WIDTH as f32;
            let v = j as f32 / IMAGE_HEIGHT as f32;

            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = ray_color(&r, &world);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

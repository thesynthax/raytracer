mod vec3;
mod ppm;
mod ray;

use vec3::{Vec3, Color, Point};
use ray::Ray;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32
{
    /*
        * if (P-C).(P-C)=r^2 where P=A+tB vector and C=center
        * so expanding,
        * t^2(B.B) + 2tB.(A-C) + (A-C).(A-C) - r^2 = 0
        * so if D > 0, then the ray has hit the sphere    
    */
    let oc: Vec3 = r.origin() - *center;
    let a: f32 = Vec3::dot(&r.direction(), &r.direction());
    let b: f32 = 2.0 * Vec3::dot(&r.direction(), &oc);
    let c: f32 = Vec3::dot(&oc, &oc) - radius*radius;

    let disc = b*b - 4.0*a*c;

    let hit_point = if disc < 0.0 { -1.0 } else { (-b - disc.sqrt()/(2.0*a)) };
    hit_point
}

fn ray_color(r: &Ray) -> Color
{
    let t: f32 = hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0
    {
        let n: Vec3 = Vec3::unit_vector(&(r.parametric_point(t) - Vec3::new(0.0,0.0,-1.0)));
        return (Color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0))/2.0;
    }

    let unit_direction = Vec3::unit_vector(&(r.direction()));
    let t: f32 = (unit_direction.y() + 1.0)/2.0;

    let white = Color::one(); //Color::zero would be black
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

    //Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH); 

    //Render
    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, MAX_VALUE);
    for j in (0..IMAGE_HEIGHT).rev()
    {
        for i in 0..IMAGE_WIDTH 
        {
            let u = i as f32 / IMAGE_WIDTH as f32;
            let v = j as f32 / IMAGE_HEIGHT as f32;

            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = ray_color(&r);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

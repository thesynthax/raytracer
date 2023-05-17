mod vec3;
mod ppm;

use vec3::Vec3;

fn main()
{
    //ppm::write_ppm(200, 100, 255);
    println!("Hello, world! {}", 1f32);

    let u = Vec3::new(3.0, 5.0, 1.0);
    let v = Vec3::new(1.0, 2.0, 6.0);

    let w = u + v;

    println!("{:?}", w);

}

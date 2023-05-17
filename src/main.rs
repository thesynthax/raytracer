mod vec3;

use vec3::Vec3;

fn write_ppm(w: i32, h: i32, max_value: i32)
{
    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev()
    {
        for i in (0..w)
        {
            let r: f32 = i as f32 / w as f32;
            let g: f32 = j as f32 / h as f32;
            let b: f32 = 0.2;
            
            let ir: i32 = (256.0*r) as i32; 
            let ig: i32 = (256.0*g) as i32; 
            let ib: i32 = (256.0*b) as i32; 

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main()
{
    write_ppm(200, 100, 255);
    //println!("Hello, world!");
}

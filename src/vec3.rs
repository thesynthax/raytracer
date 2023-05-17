use std::ops;

#[derive(Debug)]
pub struct Vec3
{
    e: [f32; 3]
}

impl Vec3 
{
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 
    {
        Vec3 
        {
            e: [e0, e1, e2]
        } 
    }
}

impl ops::Add for Vec3 
{
    type Output = Self;

    fn add(self, v2: Vec3) -> Vec3
    {
        Vec3 { e: [self.e[0]+v2.e[0], self.e[1]+v2.e[1], self.e[2]+v2.e[2]] }
    }
}

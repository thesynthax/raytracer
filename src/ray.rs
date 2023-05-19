use crate::vec3::{Vec3, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    A: Vec3,
    B: Vec3
}

impl Ray
{
    pub fn ray(a: Vec3, b: Vec3) -> Ray
    {
        Ray { A: a, B: b }
    }

    pub fn origin(self) -> Point
    {
        self.A
    }

    pub fn direction(self) -> Vec3
    {
        self.B
    }

    pub fn parametric_point(self, t: f32) -> Point
    {
        self.A + self.B*t
    }
}

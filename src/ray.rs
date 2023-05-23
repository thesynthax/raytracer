use crate::vec3::{Vec3, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    a: Vec3,
    b: Vec3
}

impl Ray
{
    pub fn ray(a: Vec3, b: Vec3) -> Ray
    {
        Ray { a, b }
    }

    pub fn origin(self) -> Point
    {
        self.a
    }

    pub fn direction(self) -> Vec3
    {
        self.b
    }

    pub fn parametric_point(self, t: f32) -> Point
    {
        self.a + self.b*t
    }
}

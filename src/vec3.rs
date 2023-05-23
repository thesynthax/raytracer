use std::ops;


#[derive(Default, Debug, Copy, Clone, PartialEq)]
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

    pub fn x(self) -> f32
    {
        self.e[0]
    }

    pub fn y(self) -> f32
    {
        self.e[1]
    }

    pub fn z(self) -> f32
    {
        self.e[2]
    }

    pub fn r(self) -> f32
    {
        self.e[0]
    }

    pub fn g(self) -> f32
    {
        self.e[1]
    }

    pub fn b(self) -> f32
    {
        self.e[2]
    }

    pub fn zero() -> Vec3
    {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn one() -> Vec3
    {
        Vec3 { e: [1.0, 1.0, 1.0] }
    }
    
    pub fn length_squared(self) -> f32
    {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(self) -> f32
    {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3
    {
        *v / (*v).length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 
    {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 
    {
        Vec3 
        {
            e: 
            [
                v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
            ]
        }
    }
}

impl ops::Add for Vec3 
{
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output //Vec3
    {
        Vec3 { e: [self.e[0]+other.e[0], self.e[1]+other.e[1], self.e[2]+other.e[2]] }
    }
}

impl ops::Sub for Vec3 
{
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output //Vec3
    {
        Vec3 { e: [self.e[0]-other.e[0], self.e[1]-other.e[1], self.e[2]-other.e[2]] }
    }
}

impl ops::Mul<f32> for Vec3
{
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output
    {
        Vec3 { e: [t*self.e[0], t*self.e[1], t*self.e[2]] }
    }
}

impl ops::Div<f32> for Vec3
{
    type Output = Self;

    fn div(self, t: f32) -> Self::Output
    {
        Vec3 { e: [self.e[0]/t, self.e[1]/t, self.e[2]/t] }
    }
}

impl ops::Neg for Vec3
{
    type Output = Self;

    fn neg(self) -> Self::Output 
    {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }    
    }
}

impl ops::AddAssign for Vec3
{
    fn add_assign(&mut self, rhs: Self) 
    {
        *self = Vec3 { e: [self.e[0]+rhs.e[0], self.e[1]+rhs.e[1], self.e[2]+rhs.e[2]] }
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

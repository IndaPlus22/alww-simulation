use std::ops::{Add, MulAssign, SubAssign};

use rand::Rng;
#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Copy,
{
    pub fn new(_x: T, _y: T) -> Self {
        Self { x: _x, y: _y }
    }

    pub fn param(&self) -> [T; 2] {
        [self.x, self.y]
    }

    pub fn multScalar(&mut self, scalar: T)
    where
        T: std::ops::MulAssign,
    {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T: num_traits::float::Float + std::ops::DivAssign + std::ops::MulAssign> Vector2<T> {
    pub fn magnitude(&self) -> T {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }
    pub fn limit(&mut self, max: T) {
        if self.magnitude() > max {
            self.normalize();
            self.multScalar(max)
        }
    }
}

impl<T> Add for Vector2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Vector2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Clone for Vector2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T> Copy for Vector2<T> where T: Copy {}

impl<T> SubAssign for Vector2<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign for Vector2<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

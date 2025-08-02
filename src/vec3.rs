pub trait Float:
    num::traits::Float
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::fmt::Display
{
}

impl Float for f32 {}
impl Float for f64 {}

#[derive(Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type FloatVec3 = Vec3<f32>;
pub type DoubleVec3 = Vec3<f64>;
pub type Point3 = DoubleVec3;

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> T {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Self {
        self / self.len()
    }
}

impl<T> Default for Vec3<T>
where
    T: Float,
{
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> std::ops::Neg for Vec3<T>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> num::traits::MulAddAssign for Vec3<T>
where
    T: Float,
{
    fn mul_add_assign(&mut self, a: Self, b: Self) {
        self.x = self.x.mul_add(a.x, b.x);
        self.y = self.y.mul_add(a.y, b.y);
        self.z = self.z.mul_add(a.z, b.z);
    }
}

impl<T> std::ops::AddAssign for Vec3<T>
where
    T: Float,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> std::ops::SubAssign for Vec3<T>
where
    T: Float,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> std::ops::MulAssign<Self> for Vec3<T>
where
    T: Float,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> std::ops::MulAssign<T> for Vec3<T>
where
    T: Float,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> std::ops::DivAssign<Self> for Vec3<T>
where
    T: Float,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<T> std::ops::DivAssign<T> for Vec3<T>
where
    T: Float,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T> std::ops::Add<Self> for &Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::Add<&Self> for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::Add<Self> for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::Sub<Self> for &Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> std::ops::Sub<&Self> for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> std::ops::Sub<Self> for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> std::ops::Mul<T> for &Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> std::ops::Mul<T> for Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<&Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: &Vec3<f32>) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<&Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: &Vec3<f64>) -> Self::Output {
        rhs * self
    }
}

impl<T> std::ops::Div<T> for &Vec3<T>
where
    T: Float,
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> std::fmt::Display for Vec3<T>
where
    T: Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

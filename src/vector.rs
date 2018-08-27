use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

pub trait FloatBound {}

impl FloatBound for f32 {}

impl FloatBound for f64 {}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Vec3<T> where
    T: FloatBound
{
    pub x: T,
    pub y: T,
    pub z: T,
}

//default
impl Default for Vec3<f32> {
    #[inline]
    fn default() -> Self {
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }	
    }
}

impl Default for Vec3<f64> {
    #[inline]
	fn default() -> Self {
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }	
    }
}

//new
impl<T> Vec3<T> where
    T: FloatBound
{
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {
            x: x,
            y: y,
            z: z,	
        }
    }
}

//Add
impl<T> Add for Vec3<T> where 
    T: Add<Output=T> + FloatBound
{
    type Output = Vec3<T>;

    #[inline]
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,	
            y: self.y + rhs.y,	
            z: self.z + rhs.z,	
        }
    } 
}

impl<T> AddAssign for Vec3<T> where
    T: Add<Output=T> + Clone + FloatBound
{
    #[inline]
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = Vec3 {
            x: self.x.clone() + rhs.x,	
            y: self.y.clone() + rhs.y,	
            z: self.z.clone() + rhs.z,	
        }
    }
}

//Sub
impl<T> Sub for Vec3<T> where
    T: Sub<Output=T> + FloatBound
{
    type Output = Vec3<T>;

    #[inline]
    fn sub(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - rhs.x,	
            y: self.y - rhs.y,	
            z: self.z - rhs.z,	
        }
    } 
}

impl<T> SubAssign for Vec3<T> where
    T: Sub<Output=T> + Clone + FloatBound
{
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        *self = Vec3 {
            x: self.x.clone() - rhs.x,	
            y: self.y.clone() - rhs.y,	
            z: self.z.clone() - rhs.z,	
        }
    }
}

//Mul
impl<T> Mul<T> for Vec3<T> where
    T: Mul<Output=T> + Clone + FloatBound
{
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x * rhs.clone(),	
            y: self.y * rhs.clone(),	
            z: self.z * rhs.clone(),	
        }
    } 
}

impl<T> MulAssign<T> for Vec3<T> where
    T: Mul<Output=T> + Clone + FloatBound
{

    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        *self = Vec3 {
            x: self.x.clone() * rhs.clone(),
            y: self.y.clone() * rhs.clone(),	
            z: self.z.clone() * rhs.clone(),	
        }
    }
}

//Div
impl<T> Div<T> for Vec3<T> where
    T: Div<Output=T> + Clone + FloatBound
{
    type Output = Vec3<T>;

    #[inline]
    fn div(self, rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x / rhs.clone(),	
            y: self.y / rhs.clone(),	
            z: self.z / rhs.clone(),	
        }
    } 
}

impl<T> DivAssign<T> for Vec3<T> where
    T: Div<Output=T> + Clone + FloatBound
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        *self = Vec3 {
            x: self.x.clone() / rhs.clone(),
            y: self.y.clone() / rhs.clone(),	
            z: self.z.clone() / rhs.clone(),	
        }
    }
}

//Distance
#[inline]
pub fn distsq<T>(vec1: &Vec3<T>, vec2: &Vec3<T>) -> T where
    T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Clone + FloatBound
{
    let dd: T = (vec1.clone().x - vec2.clone().x) * (vec1.clone().x - vec2.clone().x) + (vec1.clone().y - vec2.clone().y) * (vec1.clone().y - vec2.clone().y) + (vec1.clone().z - vec2.clone().z) * (vec1.clone().z - vec2.clone().z);
    dd
}

//Basic type conversion.
//From
//(1) f32->f64
//(2) f64->f32

//(1)
impl From<Vec3<f32>> for Vec3<f64> {
    #[inline]
    fn from(vec: Vec3<f32>) -> Self {
        Vec3 {
            x: vec.x as f64,	
            y: vec.y as f64,	
            z: vec.z as f64,	
        }
    }
}

//(2)
impl From<Vec3<f64>> for Vec3<f32> {
    #[inline]
    fn from(vec: Vec3<f64>) -> Self {
        Vec3 {
            x: vec.x as f32,	
            y: vec.y as f32,	
            z: vec.z as f32,	
        }
    }
}

//Tests
#[cfg(test)]
mod vec3_test {
    use super::*;
    #[test]
    fn basic_test(){
        let v1: Vec3<f32> = Vec3::new(0.0, 0.0, 0.0);
        let v2: Vec3<f64> = Default::default(); 
        let v3: Vec3<f64> = v1.clone().into();
        assert_eq!(v2, v3);
        let v4: Vec3<f32> = v3.into();
        assert_eq!(v1, v4);

        let v5: Vec3<f32> = Vec3::new(0.0, 0.0, 0.0);
        let v6: Vec3<f32> = Default::default(); 
        assert_eq!(v5==v5, true);
        assert_eq!(v5==v6, true);

        let sc1: f32 = 0.5;
        let sc2: f32 = 4.0;
        let v7: Vec3<f32> = Vec3::new(10.0, 10.0, 10.0);
        let v8: Vec3<f32> = Vec3::new(80.0, 80.0, 80.0);
        assert_eq!((v7 / sc1) * sc2, v8);

        let sc1: f64= 0.5;
        let sc2: f64= 4.0;
        let v9: Vec3<f64> = Vec3::new(10.0, 10.0, 10.0);
        let v10: Vec3<f64> = Vec3::new(80.0, 80.0, 80.0);
        assert_eq!((v9 / sc1) * sc2, v10);

        let v11: Vec3<f32> = Vec3::new(80.0, 80.0, 80.0);
        let v12: Vec3<f32> = Vec3::new(80.0, 80.0, 80.0);
        let v13: Vec3<f32> = Vec3::new(160.0, 160.0, 160.0);
        let v14: Vec3<f32> = Default::default();
        assert_eq!(v11 + v12 - v13, v14);

        let v15: Vec3<f64> = Vec3::new(80.0, 80.0, 80.0);
        let v16: Vec3<f64> = Vec3::new(80.0, 80.0, 80.0);
        let v17: Vec3<f64> = Vec3::new(160.0, 160.0, 160.0);
        let v18: Vec3<f64> = Default::default();
        assert_eq!(v15 + v16 - v17, v18);
    }

    #[test]
    fn distsq_test() {
        let v1: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3<f32> = Vec3::new(3.0, 2.0, 1.0);
        let result: f32 = 8.0;
        assert_eq!(distsq(&v1, &v2).sqrt(), result.sqrt());

        let v3: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let v4: Vec3<f64> = Vec3::new(3.0, 2.0, 1.0);
        let result: f64 = 8.0;
        assert_eq!(distsq(&v3, &v4).sqrt(), result.sqrt());
    }
}


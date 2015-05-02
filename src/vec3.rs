use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[test]
fn vec_constructor_works() {
    let x = 1.0f64;
    let y = 2.0f64;
    let z = 3.0f64;
    let vec = Vec3::new(x, y, z);
    assert_eq!(x, vec.x);
    assert_eq!(y, vec.y);
    assert_eq!(z, vec.z);
}

#[test]
fn vec_norm_works() {
    let mut vec = Vec3::new(1.0, 1.0, 1.0);
    vec.norm();
    let expected = 1.0 / (3.0f64).sqrt();
    assert_eq!(vec.x, expected);
    assert_eq!(vec.y, expected);
    assert_eq!(vec.z, expected);
}

#[test]
fn vec_dot_product_same_direction_is_one() {
    let direction = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(direction.dot(direction), 1.0);
}

#[test]
fn vec_dot_product_opposite_direction_is_minus_one() {
    let up = Vec3::new(0.0, 1.0, 0.0);
    let down = Vec3::new(0.0, -1.0, 0.0);
    assert_eq!(up.dot(down), -1.0);
    assert_eq!(down.dot(up), -1.0);
}

#[test]
fn vec_cross_product_test() {
    let up = Vec3::new(0.0, 1.0, 0.0);
    let right = Vec3::new(1.0, 0.0, 0.0);
    let crossed = Vec3::cross(up, right);
    assert_eq!(crossed.x, 0.0);
    assert_eq!(crossed.y, 0.0);
    assert_eq!(crossed.z, -1.0);
}

#[test]
fn vec_addition() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(7.0, 8.0, 9.0);
    let res = x + y;
    assert_eq!(res.x, 8.0);
    assert_eq!(res.y, 10.0);
    assert_eq!(res.z, 12.0);
}

#[test]
fn vec_subtraction() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let y = Vec3::new(7.0, 8.0, 9.0);
    let res = x - y;
    assert_eq!(res.x, -6.0);
    assert_eq!(res.y, -6.0);
    assert_eq!(res.z, -6.0);
}

#[test]
fn vec_multiplication_f64() {
    let x = Vec3::new(1.0, 2.0, 3.0);
    let res = x * 2.0;
    assert_eq!(res.x, 2.0);
    assert_eq!(res.y, 4.0);
    assert_eq!(res.z, 6.0);
}

#[test]
fn vec_multiplication_vec() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(5.0, 6.0, 7.0);
    let res = a * b;
    assert_eq!(res.x, 10.0);
    assert_eq!(res.y, 18.0);
    assert_eq!(res.z, 28.0);
}

#[test]
fn vec_max_coeff() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(a.max_coeff(), 4.0);
}


#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn norm(&mut self) {
        let length = self.dot(*self).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalized(v: Vec3) -> Vec3 {
        let length = v.dot(v).sqrt();
        return Vec3::new(v.x / length,
                         v.y / length,
                         v.z / length);
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        return Vec3::new(u.y * v.z - u.z * v.y,
                         u.z * v.x - u.x * v.z,
                         u.x * v.y - u.y * v.x)
    }

    pub fn max_coeff(&self) -> f64 {
        self.x.max(self.y).max(self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x,
                  self.y + _rhs.y,
                  self.z + _rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x,
                  self.y - _rhs.y,
                  self.z - _rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x,
                  self.y * _rhs.y,
                  self.z * _rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

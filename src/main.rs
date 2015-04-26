use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
extern crate rand;

#[test]
fn vec_constructor_works() {
    let x = 1.0f64;
    let y = 2.0f64;
    let z = 3.0f64;
    let vec = Vec::new(x, y, z);
    assert_eq!(x, vec.x);
    assert_eq!(y, vec.y);
    assert_eq!(z, vec.z);
}

#[test]
fn vec_norm_works() {
    let mut vec = Vec::new(1.0, 1.0, 1.0);
    vec.norm();
    let expected = 1.0 / (3.0f64).sqrt();
    assert_eq!(vec.x, expected);
    assert_eq!(vec.y, expected);
    assert_eq!(vec.z, expected);
}

#[test]
fn vec_dot_product_same_direction_is_one() {
    let direction = Vec::new(0.0, 1.0, 0.0);
    assert_eq!(direction.dot(direction), 1.0);
}

#[test]
fn vec_dot_product_opposite_direction_is_minus_one() {
    let up = Vec::new(0.0, 1.0, 0.0);
    let down = Vec::new(0.0, -1.0, 0.0);
    assert_eq!(up.dot(down), -1.0);
    assert_eq!(down.dot(up), -1.0);
}

#[test]
fn vec_cross_product_test() {
    let up = Vec::new(0.0, 1.0, 0.0);
    let right = Vec::new(1.0, 0.0, 0.0);
    let crossed = Vec::cross(up, right);
    assert_eq!(crossed.x, 0.0);
    assert_eq!(crossed.y, 0.0);
    assert_eq!(crossed.z, -1.0);
}

#[test]
fn vec_addition() {
    let x = Vec::new(1.0, 2.0, 3.0);
    let y = Vec::new(7.0, 8.0, 9.0);
    let res = x + y;
    assert_eq!(res.x, 8.0);
    assert_eq!(res.y, 10.0);
    assert_eq!(res.z, 12.0);
}

#[test]
fn vec_subtraction() {
    let x = Vec::new(1.0, 2.0, 3.0);
    let y = Vec::new(7.0, 8.0, 9.0);
    let res = x - y;
    assert_eq!(res.x, -6.0);
    assert_eq!(res.y, -6.0);
    assert_eq!(res.z, -6.0);
}

#[test]
fn vec_multiplication_f64() {
    let x = Vec::new(1.0, 2.0, 3.0);
    let res = x * 2.0;
    assert_eq!(res.x, 2.0);
    assert_eq!(res.y, 4.0);
    assert_eq!(res.z, 6.0);
}

#[derive(Clone, Copy)]
struct Vec {
    x: f64,
    y: f64,
    z: f64
}

impl Vec {
    fn new(x: f64, y: f64, z: f64) -> Vec {
        Vec { x: x, y: y, z: z }
    }

    fn norm(&mut self) {
        let length = self.dot(*self).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    fn normalized(v: Vec) -> Vec {
        let length = v.dot(v).sqrt();
        return Vec::new(v.x / length,
                        v.y / length,
                        v.z / length);
    }

    fn dot(&self, other: Vec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(u: Vec, v: Vec) -> Vec {
        return Vec::new(u.y * v.z - u.z * v.y,
                        u.z * v.x - u.x * v.z,
                        u.x * v.y - u.y * v.x)
    }
}

impl Add for Vec {
    type Output = Vec;

    fn add(self, _rhs: Vec) -> Vec {
        Vec::new(self.x + _rhs.x,
                 self.y + _rhs.y,
                 self.z + _rhs.z)
    }
}

impl Sub for Vec {
    type Output = Vec;

    fn sub(self, _rhs: Vec) -> Vec {
        Vec::new(self.x - _rhs.x,
                 self.y - _rhs.y,
                 self.z - _rhs.z)
    }
}

impl Mul<f64> for Vec {
    type Output = Vec;

    fn mul(self, rhs: f64) -> Vec {
        // add an i32 to a Point and get an f64
        return Vec::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}




#[test]
fn ray_has_origin_and_destination() {
    let origin = Vec::new(1.0, 2.0, 3.0);
    let direction = Vec::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin.x, origin.x);
    assert_eq!(ray.direction.x, direction.x);
}



struct Ray {
    origin: Vec,
    direction: Vec
}

impl Ray {
    fn new(origin: Vec, direction: Vec) -> Ray {
        Ray { origin: origin, direction: direction }
    }
}


fn tent_filter() -> f64 {
    let r1 : f64 = 2.0 * rand::random::<f64>();
    if r1 < 1.0 {
        return r1.sqrt() - 1.0
    } else {
        return 1.0 - (2.0 - r1).sqrt()
    }
}

fn radiance(r: Ray, depth: i32) -> Vec {
    Vec::new(0.0, 0.0, 0.0)
}

#[cfg(not(test))]
fn main() {
    let w = 512;
    let h = 384;
    let samps = 500/4;

    // Camera position
    let cam = Ray::new(Vec::new(50.0, 52.0, 295.6),
                       Vec::normalized(Vec::new(0.0, -0.042612, -1.0)));
    // x direction increment
    let cx = Vec::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    // y diraction increment
    let cy = Vec::normalized(Vec::cross(cx, cam.direction)) * 0.5135;
    

    for y in 0..h { // Loop over image rows
        for x in 0..w { // Loop over image cols
            for sy in 0..2 { // 2x2 subpixel rows
                for sx in 0..2 { // 2x2 subpixel cols
                    let mut r = Vec::new(0.0, 0.0, 0.0);
                    for _ in 0..samps {
                        let dx = tent_filter();
                        let dy = tent_filter();
                        let d : Vec =
                            cx * (((sx as f64 + 0.5 + dx) / 2.0 + x as f64) / w as f64 - 0.5) +
                            cy * (((sy as f64 + 0.5 + dy) / 2.0 + y as f64) / h as f64 - 0.5) + 
                            cam.direction;
                        let ray = Ray::new(cam.origin + d * 140.0,
                                           Vec::normalized(d));
                        r = r + radiance(ray, 0) * (1.0 / samps as f64);
                    }
                }
            }
        }
    }
}

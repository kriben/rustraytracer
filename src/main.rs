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

    fn dot(&self, other: Vec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}

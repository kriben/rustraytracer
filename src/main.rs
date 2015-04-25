
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
        let length = (self.x * self.x +
                      self.y * self.y +
                      self.z * self.z).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }
}


#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}

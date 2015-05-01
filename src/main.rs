pub mod vec3;
use vec3::Vec3;
extern crate rand;

#[test]
fn ray_has_origin_and_destination() {
    let origin = Vec3::new(1.0, 2.0, 3.0);
    let direction = Vec3::new(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin.x, origin.x);
    assert_eq!(ray.direction.x, direction.x);
}


#[derive(Clone, Copy)]
struct Ray {
    origin: vec3::Vec3,
    direction: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction }
    }
}


#[test]
fn sphere_constructor_works() {
    let radius = 63.0;
    let position = Vec3::new(1.0, 2.0, 3.0);
    let emission = Vec3::new(4.0, 5.0, 6.0);
    let color = Vec3::new(255.0, 0.0, 0.0);
    let sphere = Sphere::new(radius, position, emission, color);
    assert_eq!(sphere.radius, radius);
}

#[test]
fn sphere_ray_intersection_when_missing() {
    // Place a sphere in origin and cast a ray away from it
    let position = Vec3::new(0.0, 0.0, 0.0);
    let emission = Vec3::new(4.0, 5.0, 6.0);
    let color = Vec3::new(255.0, 0.0, 0.0);
    let sphere = Sphere::new(1.0, position, emission, color);

    let ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(sphere.intersect(ray).is_none(), true);
}

#[test]
fn sphere_ray_intersection_when_hitting() {
    // Place a sphere in origin and cast a ray away from it
    let position = Vec3::new(0.0, 0.0, 0.0);
    let emission = Vec3::new(4.0, 5.0, 6.0);
    let color = Vec3::new(255.0, 0.0, 0.0);
    let sphere = Sphere::new(1.0, position, emission, color);

    let ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    assert_eq!(sphere.intersect(ray), Some(2.0));
}


#[derive(Clone, Copy)]
struct Sphere {
    radius: f64,
    position: Vec3,
    emission: Vec3,
    color: Vec3
}

impl Sphere {
    fn new(radius: f64, position: Vec3, emission: Vec3, color: Vec3) -> Sphere {
        Sphere { radius: radius, position: position, emission: emission, color: color }
    }

    fn intersect(self: Sphere, ray: Ray) -> Option<f64> {
        // Solve t^2*d.d + 2*t*(o-p).d + (o-p).(o-p)-R^2 = 0
        let op: Vec3 = self.position - ray.origin;
        let eps = 1.0e-4;
        let b : f64 = op.dot(ray.direction);
        let det : f64 = b * b - op.dot(op) + self.radius * self.radius;

        if det < 0.0 {
            return None;
        }

        let det2 = det.sqrt();

        if b - det2 > eps {
            return Some(b - det2);
        }
        else if b + det2 > eps {
            return Some(b + det2);
        }

        None
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

fn intersect(ray: Ray, spheres: &Vec<Sphere>) -> Option<(usize, f64)> {
    let mut idx = -1;
    let mut smallest = std::f64::MAX;
    let mut found = false;
    for (i, sphere) in spheres.iter().enumerate() {
        match sphere.intersect(ray) {
            Some(distance) => {
                if distance < smallest {
                    smallest = distance;
                    idx = i;
                    found = true;
                }
            }
            None => {}
        }
    }

    if found {
        Some((idx, smallest))
    }
    else {
        None
    }
}

fn radiance(r: Ray, depth: i32, spheres: &Vec<Sphere>) -> Vec3 {

    Vec3::new(spheres[0].radius, 0.0, 0.0)
}

#[cfg(not(test))]
fn main() {
    let w = 512;
    let h = 384;
    let samps = 500/4;


    let spheres= vec![
        Sphere::new(1e5, Vec3::new(1e5, 40.8, 81.6), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75,0.25,0.25))
            ];


    // Camera position
    let cam = Ray::new(Vec3::new(50.0, 52.0, 295.6),
                       Vec3::normalized(Vec3::new(0.0, -0.042612, -1.0)));
    // x direction increment
    let cx = Vec3::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    // y diraction increment
    let cy = Vec3::normalized(Vec3::cross(cx, cam.direction)) * 0.5135;


    for y in 0..h { // Loop over image rows
        for x in 0..w { // Loop over image cols
            for sy in 0..2 { // 2x2 subpixel rows
                for sx in 0..2 { // 2x2 subpixel cols
                    let mut r = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samps {
                        let dx = tent_filter();
                        let dy = tent_filter();
                        let d : Vec3 =
                            cx * (((sx as f64 + 0.5 + dx) / 2.0 + x as f64) / w as f64 - 0.5) +
                            cy * (((sy as f64 + 0.5 + dy) / 2.0 + y as f64) / h as f64 - 0.5) +
                            cam.direction;
                        let ray = Ray::new(cam.origin + d * 140.0,
                                           Vec3::normalized(d));
                        r = r + radiance(ray, 0, &spheres) * (1.0 / samps as f64);
                    }
                }
            }
        }
    }
}

pub mod vec3;
use vec3::Vec3;
use std::path::Path;
extern crate rand;
extern crate image;

use image::{
    GenericImage,
    ImageBuffer
};


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
    let sphere = Sphere::new(radius, position, emission, color,
                             Reflection::DIFF);
    assert_eq!(sphere.radius, radius);
}

#[test]
fn sphere_ray_intersection_when_missing() {
    // Place a sphere in origin and cast a ray away from it
    let position = Vec3::new(0.0, 0.0, 0.0);
    let emission = Vec3::new(4.0, 5.0, 6.0);
    let color = Vec3::new(255.0, 0.0, 0.0);
    let sphere = Sphere::new(1.0, position, emission, color, Reflection::DIFF);

    let ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    assert_eq!(sphere.intersect(ray).is_none(), true);
}

#[test]
fn sphere_ray_intersection_when_hitting() {
    // Place a sphere in origin and cast a ray away from it
    let position = Vec3::new(0.0, 0.0, 0.0);
    let emission = Vec3::new(4.0, 5.0, 6.0);
    let color = Vec3::new(255.0, 0.0, 0.0);
    let sphere = Sphere::new(1.0, position, emission, color, Reflection::DIFF);

    let ray = Ray::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    assert_eq!(sphere.intersect(ray), Some(2.0));
}

#[derive(Clone, Copy)]
enum Reflection {
    DIFF,
    SPEC,
    REFR
}

#[derive(Clone, Copy)]
struct Sphere {
    radius: f64,
    position: Vec3,
    emission: Vec3,
    color: Vec3,
    reflection: Reflection
}

impl Sphere {
    fn new(radius: f64, position: Vec3, emission: Vec3,
           color: Vec3, reflection: Reflection) -> Sphere {
        Sphere { radius: radius,
                 position: position,
                 emission: emission,
                 color: color,
                 reflection: reflection }
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

fn make_perpendicular_vec3(w: Vec3) -> Vec3 {
    if w.x.abs() > 0.1 {
        Vec3::cross(Vec3::new(0.0, 1.0, 0.0), w)
    }
    else {
        Vec3::cross(Vec3::new(1.0, 0.0, 0.0), w)
    }
}

fn radiance(ray: Ray, depth: i32, spheres: &Vec<Sphere>) -> Vec3 {
    let x = intersect(ray, spheres);
    if x.is_none() {
        // If miss return black
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let (id, t) = x.unwrap();
    let x = ray.origin + ray.direction * t;
    let n = Vec3::normalized(x - spheres[id].position);
    let nl = if n.dot(ray.direction) < 0.0 { n } else { n * -1.0 };
    let mut f = spheres[id].color;
    // Max reflection
    let p = f.max_coeff();

    if depth + 1 > 5 {
        if rand::random::<f64>() < p {
            f = f * (1.0 / p);
        }
        else {
            return spheres[id].emission; //R.R.
        }
    }

    match spheres[id].reflection {
        Reflection::DIFF => {
            // Ideal DIFFUSE reflection
            // Random angle
            let r1 = 2.0 * std::f64::consts::PI * rand::random::<f64>();
            // Distance from center
            let r2 = rand::random::<f64>();
            let r2s = r2.sqrt();
            // Normal
            let w : Vec3 = nl;
            // Make u perpedicular to w
            let u : Vec3 = Vec3::normalized(make_perpendicular_vec3(w));
            // u is perpendicular to w
            let v : Vec3 = Vec3::cross(w, u);
            // Random reflection ray
            let d : Vec3 = Vec3::normalized(u*r1.cos()*r2s + v*r1.sin()*r2s + w*(1.0-r2).sqrt());
            return spheres[id].emission + (radiance(Ray::new(x, d), depth + 1, spheres) * f);
        }
        Reflection::SPEC => {
            let new_ray = Ray::new(x, ray.direction - n * 2.0 * n.dot(ray.direction));
            return spheres[id].emission + (radiance(new_ray, depth + 1, spheres) * f);
        }
        Reflection::REFR => {
            // Not implemented yet
            return Vec3::new(1.0, 0.0, 0.0);
        }
    }
}

#[cfg(not(test))]
fn main() {
    let w = 512;
    let h = 384;
    let samps = 40/4;

    let spheres= vec![
        // Left
        Sphere::new(1e5,
                    Vec3::new(1.0e5 + 1.0, 40.8, 81.6),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.75,0.25,0.25),
                    Reflection::DIFF),
        // Right
        Sphere::new(1.0e5,
                    Vec3::new(-1.0e5+99.0, 40.8, 81.6),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.25,0.25,0.75),
                    Reflection::DIFF),
        // Back
        Sphere::new(1e5,
                    Vec3::new(50.0, 40.8, 1.0e5),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.75, 0.75, 0.75),
                    Reflection::DIFF),
        // Front
        Sphere::new(1e5,
                    Vec3::new(50.0, 40.8, -1.0e5 + 170.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Reflection::DIFF),
        // Bottom
        Sphere::new(1e5,
                    Vec3::new(50.0, 1.0e5, 81.6),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.75, 0.75, 0.75),
                    Reflection::DIFF),
        // Top
        Sphere::new(1e5,
                    Vec3::new(50.0, -1.0e5 + 81.6, 81.6),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.75, 0.75, 0.75),
                    Reflection::DIFF),
        // Mirror
        Sphere::new(16.5,
                    Vec3::new(27.0, 16.5, 47.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0) * 0.999,
                    Reflection::SPEC),
        // Light
        Sphere::new(600.0,
                    Vec3::new(50.0, 681.6 - 0.27, 81.6),
                    Vec3::new(12.0, 12.0, 12.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Reflection::DIFF)
            ];


    // Camera position
    let cam = Ray::new(Vec3::new(50.0, 52.0, 295.6),
                       Vec3::normalized(Vec3::new(0.0, -0.042612, -1.0)));
    // x direction increment
    let cx = Vec3::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    // y diraction increment
    let cy = Vec3::normalized(Vec3::cross(cx, cam.direction)) * 0.5135;

    let mut image = Vec::with_capacity(h * w);
    for y in 0..h { // Loop over image rows
        for x in 0..w { // Loop over image cols
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
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
                    pixel = pixel + Vec3::clamp(r, 0.0, 1.0) * 0.25;
                }
            }
            image.push(pixel);
        }
    }

    save_image(w as u32, h as u32, &image);
}

fn to_int(input : f64) -> u8 {
    let x = input.min(1.0).max(0.0);
    return (x.powf(1.0/2.2)*255.0 + 0.5) as u8;
}

fn save_image(width: u32, height: u32, pixels: &Vec<Vec3>) {
    //Construct a new ImageBuffer with the specified width and height.
    let mut img = ImageBuffer::new(width, height);
    for y in 0..height { // Loop over image rows
        for x in 0..width { // Loop over image cols
            //Put a pixel at coordinate (100, 100)
            let idx = (y * width + x) as usize;

            img.put_pixel(x, height - 1 - y,
                          image::Rgb([to_int(pixels[idx].x),
                                      to_int(pixels[idx].y),
                                      to_int(pixels[idx].z)]));
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(Path::new("test.png"));
}

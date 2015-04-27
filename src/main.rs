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



struct Ray {
    origin: vec3::Vec3,
    direction: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
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

fn radiance(r: Ray, depth: i32) -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

#[cfg(not(test))]
fn main() {
    let w = 512;
    let h = 384;
    let samps = 500/4;

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
                        r = r + radiance(ray, 0) * (1.0 / samps as f64);
                    }
                }
            }
        }
    }
}

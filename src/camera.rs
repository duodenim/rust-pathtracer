use vec3::Vec3;
use ray::Ray;

extern crate rand;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v*rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}

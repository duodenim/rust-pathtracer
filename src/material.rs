use ray::Ray;
use vec3::Vec3;
use sphere::Hit;

extern crate rand;

pub struct ScatterRecord {
    pub should_scatter: bool,
    pub attenuation: Vec3,
    pub scattered: Ray
}
pub trait Material {
    fn scatter(&self, r: &Ray, hit_record: &Hit) -> ScatterRecord;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, hit_record: &Hit) -> ScatterRecord {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        ScatterRecord {
            should_scatter: true,
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.p, target - hit_record.p)
        }
    }
}

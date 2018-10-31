use ray::Ray;
use vec3::Vec3;

extern crate rand;

pub struct ScatterRecord {
    pub should_scatter: bool,
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, r: &Ray, t: f32, point: Vec3, normal: Vec3) -> ScatterRecord;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz
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

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, _t: f32, point: Vec3, normal: Vec3) -> ScatterRecord {
        let target = point + normal + random_in_unit_sphere();
        ScatterRecord {
            should_scatter: true,
            attenuation: self.albedo,
            scattered: Ray::new(point, target - point)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, _t: f32, point: Vec3, normal: Vec3) -> ScatterRecord {
        let reflected = reflect(Vec3::unit_vector(r.direction()), normal);

        let scattered = Ray::new(point, reflected + self.fuzz*random_in_unit_sphere());
        if scattered.direction().dot(normal) > 0.0 {
            ScatterRecord {
                should_scatter: true,
                attenuation: self.albedo,
                scattered
            }
        } else {
            ScatterRecord {
                should_scatter: false,
                attenuation: Vec3::zero_vector(),
                scattered
            }
        }
    }
}

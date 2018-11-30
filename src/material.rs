use ray::Ray;
use vec3::Vec3;
use texture::Texture;

extern crate rand;

pub struct ScatterRecord {
    pub should_scatter: bool,
    pub attenuation: Vec3,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, r: &Ray, t: f32, point: Vec3, normal: Vec3) -> ScatterRecord;
}

pub struct Lambertian {
    albedo: Box<Texture + Sync>
}

pub struct Metal {
    albedo: Box<Texture + Sync>,
    fuzz: f32
}

pub struct Dielectric {
    ref_idx: f32
}

pub struct Isotropic {
    albedo: Box<Texture + Sync>
}

impl Lambertian {
    pub fn new(albedo: Box<Texture + Sync>) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Metal {
    pub fn new(albedo: Box<Texture + Sync>, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz
        }
    }
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx
        }
    }
}

impl Isotropic {
    pub fn new(albedo: Box<Texture + Sync>) -> Isotropic {
        Isotropic {
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

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * v.dot(n) * n)
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0*r0;
    r0 + (1.0-r0)* ((1.0 - cosine).powf(5.0))
}

struct RefractRecord {
    should_refract: bool,
    refracted: Vec3
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> RefractRecord {
    let uv = Vec3::unit_vector(v);
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        return RefractRecord {
            should_refract: true,
            refracted: ni_over_nt * (uv - n*dt) - n*discriminant.sqrt()
        }
    }

    RefractRecord {
        should_refract: false,
        refracted: Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, _t: f32, point: Vec3, normal: Vec3) -> ScatterRecord {
        let target = point + normal + random_in_unit_sphere();
        ScatterRecord {
            should_scatter: true,
            attenuation: self.albedo.value(0.0, 0.0, &point),
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
                attenuation: self.albedo.value(0.0, 0.0, &point),
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

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, _t: f32, point: Vec3, normal: Vec3) -> ScatterRecord {
        let outward_normal: Vec3;
        let reflected = reflect(r.direction(), normal);
        let ni_over_nt: f32;
        let cosine: f32;
        let reflect_prob: f32;
        let scattered: Ray;

        if r.direction().dot(normal) > 0.0 {
            outward_normal = -1.0 * normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r.direction().dot(normal) / r.direction().length();
        } else {
            outward_normal = normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0 * r.direction().dot(normal) / r.direction().length();
        }
        let refract_rec = refract(r.direction(), outward_normal, ni_over_nt);
        if refract_rec.should_refract {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if rand::random::<f32>() < reflect_prob {
            scattered = Ray::new(point, reflected);
        } else {
            scattered = Ray::new(point, refract_rec.refracted);
        }

        ScatterRecord {
            should_scatter: true,
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _r: &Ray, _t: f32, point: Vec3, _normal: Vec3) -> ScatterRecord {
        ScatterRecord {
            should_scatter: true,
            attenuation: self.albedo.value(0.0, 0.0, &point),
            scattered: Ray::new(point, random_in_unit_sphere())
        }
    }
}

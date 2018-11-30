use vec3::Vec3;
use ray::Ray;
use material::Material;
use hitable::Hit;
use hitable::Hitable;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material + Sync>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<Material + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Hit {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Hit {
                    hit: true,
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: Some(&self.material)
                }
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Hit {
                    hit: true,
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                    material: Some(&self.material)
                }
            }
        }
        Hit {
            hit: false,
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None
        }
    }
}

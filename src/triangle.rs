use vec3::Vec3;
use ray::Ray;
use aabb::AABB;
use material::Material;
use hitable::Hitable;
use hitable::Hit;

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Sync>
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3, normal: Vec3, material: Box<dyn Material + Sync>) -> Triangle {
        Triangle {
            p1,
            p2,
            p3,
            normal,
            material
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit> {
        const EPSILON: f32 = 0.0000001;
        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = r.direction().cross(edge2);
        let a = edge1.dot(h);

        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin() - self.p1;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * r.direction().dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t > EPSILON && t < t_max && t > t_min {
            let normal = if r.direction().dot(self.normal) >= 0.0 {
                -1.0 * self.normal
            } else {
                self.normal
            };
            return Some(Hit {
                t,
                p: r.origin() + t * r.direction(),
                normal,
                material: &self.material
            });
        }

        None
    }
    fn bounding_box(&self) -> AABB {
        let delta = 0.001;
        let max_x = self.p1.x().max(self.p2.x().max(self.p3.x()));
        let max_y = self.p1.y().max(self.p2.y().max(self.p3.y()));
        let max_z = self.p1.z().max(self.p2.z().max(self.p3.z()));
        let min_x = self.p1.x().min(self.p2.x().min(self.p3.x()));
        let min_y = self.p1.y().min(self.p2.y().min(self.p3.y()));
        let min_z = self.p1.z().min(self.p2.z().min(self.p3.z()));
        let max = Vec3::new(max_x + delta, max_y + delta, max_z + delta);
        let min = Vec3::new(min_x - delta, min_y - delta, min_z - delta);
        AABB::new(min, max)
    }
}
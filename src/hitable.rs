use texture::Texture;
use vec3::Vec3;
use material::Material;
use ray::Ray;
use material::Isotropic;

extern crate rand;

pub struct Hit<'a> {
    pub hit: bool,
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<&'a Box<Material + Sync>>
}

impl<'a> Hit<'a> {
    pub fn no_hit() -> Hit<'a> {
        Hit {
            hit: false,
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None
        }
    }
}

pub trait Hitable {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Hit;
}

pub struct ConstantMedium {
    boundary: Box<Hitable + Sync>,
    density: f32,
    material: Box<Material + Sync>
}

impl ConstantMedium {
    pub fn new(boundary: Box<Hitable + Sync>, density: f32, texture: Box<Texture + Sync>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            density,
            material: Box::new(Isotropic::new(texture))
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Hit {
        let mut hit1 = self.boundary.hit(-1000.0, 1000.0, r);
        if hit1.hit {
            let mut hit2 = self.boundary.hit(hit1.t + 0.0001, 1000.0, r);

            if hit2.hit {
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }
                if hit1.t >= hit2.t {
                    return Hit::no_hit();
                }
                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let distance_inside_boundary = (hit2.t - hit1.t) * r.direction().length();
                let hit_distance = (-1.0/self.density) * rand::random::<f32>().ln();

                //println!("Distance inside boundary: {}, Hit distance: {}", distance_inside_boundary, hit_distance);
                if hit_distance < distance_inside_boundary {
                    let t = hit1.t + (hit_distance / r.direction().length());
                    return Hit {
                        hit: true,
                        t,
                        p: r.point_at_parameter(t),
                        normal: Vec3::new(1.0, 0.0, 0.0), //arbitrary vector
                        material: Some(&self.material)
                    };
                }
            }
        }
        Hit::no_hit()
    }
}

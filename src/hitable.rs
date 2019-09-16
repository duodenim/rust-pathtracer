use aabb::{AABB, BoxAxis};
use aabb::surrounding_bbox;
use texture::Texture;
use vec3::Vec3;
use material::Material;
use ray::Ray;
use material::Isotropic;
use std::cmp::Ordering;

extern crate rand;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<(dyn Material + Sync)>
}

pub trait Hitable {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit>;
    fn bounding_box(&self) -> AABB;
}

pub struct ConstantMedium {
    boundary: Box<dyn Hitable + Sync>,
    density: f32,
    material: Box<dyn Material + Sync>
}

pub struct BvhNode {
    left: Box<dyn Hitable + Sync>,
    right: Option<Box<dyn Hitable + Sync>>,
    bbox: AABB
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hitable + Sync>, density: f32, texture: Box<dyn Texture + Sync>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            density,
            material: Box::new(Isotropic::new(texture))
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit> {
        let hit1 = self.boundary.hit(-1000.0, 1000.0, r);
        if hit1.is_some() {
            let mut hit1 = hit1.unwrap();
            let hit2 = self.boundary.hit(hit1.t + 0.0001, 1000.0, r);

            if hit2.is_some() {
                let mut hit2 = hit2.unwrap();
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }
                if hit1.t >= hit2.t {
                    return None;
                }
                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let distance_inside_boundary = (hit2.t - hit1.t) * r.direction().length();
                let hit_distance = (-1.0/self.density) * rand::random::<f32>().ln();

                //println!("Distance inside boundary: {}, Hit distance: {}", distance_inside_boundary, hit_distance);
                if hit_distance < distance_inside_boundary {
                    let t = hit1.t + (hit_distance / r.direction().length());
                    return Some(Hit {
                        t,
                        p: r.point_at_parameter(t),
                        normal: Vec3::new(1.0, 0.0, 0.0), //arbitrary vector
                        material: &self.material
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

impl Hitable for BvhNode {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit> {
        let bbox_hit = self.bbox.hit(r, t_min, t_max);
        if bbox_hit {
            let left_hit = self.left.hit(t_min, t_max, r);

            let right_hit = match self.right {
                Some(ref x) => x.hit(t_min, t_max, r),
                None => None
            };
            match (&left_hit, &right_hit) {
                (Some(lh), Some(rh)) if lh.t < rh.t => {
                    return left_hit;
                },
                (Some(_), Some(_)) => {
                    return right_hit;
                },
                (Some(_), None) => {
                    return left_hit;
                },
                (None, Some(_)) => {
                    return right_hit;
                },
                _ => {
                    return None
                }
            }
        }
        None

    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl BvhNode {
    pub fn new(mut list: Vec<Box<dyn Hitable + Sync>>) -> BvhNode {
        let n = list.len();
        if n == 0 {
            println!("asdf");
        }
        let mut boxes = Vec::new();
        let mut left_area = vec![0.0; n];
        let mut right_area = vec![0.0; n];
        
        let main_bbox = list.iter().fold(list[0].bounding_box(), | acc, l | {
            surrounding_bbox(acc, l.bounding_box())
        });

        let axis = main_bbox.longest_axis();
        match axis {
            BoxAxis::X => {
                list.sort_by(|a, b| {
                    let left_bbox = a.bounding_box();
                    let right_bbox = b.bounding_box();

                    if left_bbox.min().x() < right_bbox.min().x() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            },
            BoxAxis::Y => {
                list.sort_by(|a, b| {
                    let left_bbox = a.bounding_box();
                    let right_bbox = b.bounding_box();

                    if left_bbox.min().y() < right_bbox.min().y() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            },
            BoxAxis::Z => {
                list.sort_by(|a, b| {
                    let left_bbox = a.bounding_box();
                    let right_bbox = b.bounding_box();

                    if left_bbox.min().z() < right_bbox.min().z() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
            }
        }

        for l in list.iter() {
            boxes.push(l.bounding_box());
        }

        left_area[0] = boxes[0].surface_area();
        let mut left_bbox = boxes[0];
        for i in 1..(n - 1) {
            left_bbox = surrounding_bbox(left_bbox, boxes[i]);
            left_area[i] = left_bbox.surface_area();
        }

        right_area[n - 1] = boxes[n - 1].surface_area();
        let mut right_bbox = boxes[n - 1];
        for i in (1..(n-1)).rev() {
            right_bbox = surrounding_bbox(right_bbox, boxes[i]);
            right_area[i] = right_bbox.surface_area();
        }

        let mut min_sah = std::f32::MAX;
        let mut min_sah_idx = 0;
        for i in 0..(n-1) {
            let sah = i as f32 * left_area[i] + (n - i - 1) as f32 * right_area[i + 1];
            if sah < min_sah {
                min_sah_idx = i;
                min_sah = sah;
            }
        }

        if list.len() == 1 {
            let hitable = list.remove(0);
            let bbox = hitable.bounding_box();
            BvhNode {
                left: hitable,
                right: None,
                bbox
            }
        } else if list.len() == 2 {
            let right = list.remove(1);
            let left = list.remove(0);
            let left_bbox = left.bounding_box();
            let right_bbox = right.bounding_box();
            BvhNode {
                left,
                right: Some(right),
                bbox: surrounding_bbox(left_bbox, right_bbox)
            }
        } else {
            let right = list.split_off(min_sah_idx + 1);
            let left = Box::new(BvhNode::new(list));
            let right = Box::new(BvhNode::new(right));
            let left_bbox = left.bounding_box();
            let right_bbox = right.bounding_box();
            BvhNode {
                left: left,
                right: Some(right),
                bbox: surrounding_bbox(left_bbox, right_bbox)
            }
        }
    }
}

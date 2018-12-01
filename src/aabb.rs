use ray::Ray;
use vec3::Vec3;
use std::mem;

#[derive(Copy, Clone)]
pub struct AABB {
    min: Vec3,
    max: Vec3
}

fn check_one_direction(ray_direction: f32, ray_origin: f32, min_axis: f32, max_axis: f32, tmin: f32, tmax: f32) -> bool {
    let inv_d = 1.0 / ray_direction;
    let mut t0 = (min_axis - ray_origin) * inv_d;
    let mut t1 = (max_axis - ray_origin) * inv_d;
    if inv_d < 0.0 {
        mem::swap(&mut t0, &mut t1);
    }
    let tmin = if t0 > tmin {
        t0
    } else {
        tmin
    };
    let tmax = if t1 < tmax {
        t1
    } else {
        tmax
    };
    if tmax <= tmin {
        return false;
    }
    true
}

pub fn surrounding_bbox(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(box0.min().x().min(box1.min.x()), box0.min.y().min(box1.min.y()), box0.min.z().min(box1.min.z()));
    let big = Vec3::new(box0.max.x().max(box1.max.x()), box0.max.y().max(box1.max.y()), box0.max.z().max(box1.max.z()));
    AABB::new(small, big)
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB {
            min,
            max
        }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        if !check_one_direction(r.direction().x(), r.origin().x(), self.min.x(), self.max.x(), tmin, tmax) {
            return false;
        }
        if !check_one_direction(r.direction().y(), r.origin().y(), self.min.y(), self.max.y(), tmin, tmax) {
            return false;
        }
        if !check_one_direction(r.direction().z(), r.origin().z(), self.min.z(), self.max.z(), tmin, tmax) {
            return false;
        }
        true
    }
}

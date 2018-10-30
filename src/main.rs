use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

extern crate png;
use png::HasParameters;

extern crate rand;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;
use sphere::Hit;

mod material;
use material::Material;
use material::Lambertian;

fn color(r : &Ray, world: &[Sphere], depth: u32) -> Vec3 {
    let mut closest_so_far = 50.0;
    let mut hit_rec = Hit::no_hit();
    for sphere in world {
        let hit = sphere.hit(0.001, closest_so_far, r);
        if hit.hit {
            closest_so_far = hit.t;
            hit_rec = hit;
        }
    }
    if hit_rec.hit {
        let material = hit_rec.material.unwrap();
        let scatter_rec = material.scatter(r, &hit_rec);
        if scatter_rec.should_scatter && depth < 50 {
            return scatter_rec.attenuation * color(&scatter_rec.scattered, world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let image_width = 400;
    let image_height = 200;
    let samples_per_pixel = 100;

    let x = Vec3::zero_vector();
    let y = Vec3::new(1.0, 2.0, 3.0);
    let z = x + y;
    let mut w = x - y;

    println!("squared length is {}", z.squared_length());
    println!("length is {}", z.length());

    println!("squared length is {}", w.squared_length());
    println!("length is {}", w.length());

    w.normalize();

    println!("squared length is {}", w.squared_length());
    println!("length is {}", w.length());

    let unit_x = Vec3::new(1.0, 0.0, 0.0);
    let unit_y = Vec3::new(0.0, 1.0, 0.0);
    let unit_z = Vec3::new(0.0, 0.0, 1.0);

    let x_cross_y = unit_x.cross(unit_y);
    let x_cross_z = unit_x.cross(unit_z);
    let y_cross_z = unit_y.cross(unit_z);

    println!("X cross Y is {:?}", x_cross_y);
    println!("X cross Z is {:?}", x_cross_z);
    println!("Y cross Z is {:?}", y_cross_z);

    //Generate world
    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vec3::new(0.8, 0.3, 0.3))));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vec3::new(0.8, 0.8, 0.0))));

    //Generate image
    let mut data = Vec::new();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero_vector();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut avg_color = Vec3::zero_vector();

            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rand::random::<f32>()) / image_width as f32;
                let v = (y as f32 + rand::random::<f32>()) / image_height as f32;

                let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

                let col = color(&r, &world, 0);

                avg_color = avg_color + (col / samples_per_pixel as f32);
            }

            //Do gamma correction
            avg_color = Vec3::new(avg_color.x().sqrt(), avg_color.y().sqrt(), avg_color.z().sqrt());

            let ir = (255.99*avg_color.x()) as u8;
            let ig = (255.99*avg_color.y()) as u8;
            let ib = (255.99*avg_color.z()) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
            data.push(255);
        }
    }

    //Store image to file
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(&data).unwrap();
}

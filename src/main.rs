use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

extern crate png;
use png::HasParameters;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;

fn color(r : &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let hit = sphere.hit(0.0, 50.0, r);
    if hit.hit {
        return 0.5 * Vec3::new(hit.normal.x() + 1.0, hit.normal.y() + 1.0, hit.normal.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let image_width = 400;
    let image_height = 200;

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

    //Generate image
    let mut data = Vec::new();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero_vector();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f32 / image_width as f32;
            let v = y as f32 / image_height as f32;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            let color = color(&r);
            let ir = (255.99*color.x()) as u8;
            let ig = (255.99*color.y()) as u8;
            let ib = (255.99*color.z()) as u8;

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

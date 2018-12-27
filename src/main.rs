use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::env;

extern crate png;
use png::HasParameters;

extern crate rand;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
use hitable::Hitable;
use hitable::BvhNode;

mod sphere;

mod triangle;
use triangle::Triangle;

mod material;
use material::Lambertian;

mod camera;
use camera::Camera;

mod texture;
use texture::ConstantTexture;

mod aabb;

extern crate rayon;
use rayon::prelude::*;

extern crate tobj;

fn color(r : &Ray, world: &Box<Hitable + Sync>, depth: u32) -> Vec3 {
    let hit_rec = world.hit(0.001, 50.0, r);
    if hit_rec.hit {
        let material = hit_rec.material.unwrap();
        let normal = hit_rec.normal;
        let point = hit_rec.p;
        let t = hit_rec.t;
        let scatter_rec = material.scatter(r, t, point, normal);
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
    let image_width = 480;
    let image_height = 270;
    let samples_per_pixel = 100;

    let args: Vec<String> = env::args().collect();

    let mut filename = "";
    if args.len() < 2 {
        println!("Usage: rust-pathtracer path_to_obj_file.obj");
        std::process::exit(0);
    } else {
        filename = &args[1];
        println!("Rendering {}....", filename);
    }

    //Save start time
    let start_time = std::time::Instant::now();

    //Generate world as seen in Chapter 12
    let mut world: Vec<Box<Hitable + Sync>> = Vec::new();

    let (models, _materials) = tobj::load_obj(Path::new(filename)).unwrap();
    for model in models.iter() {
        let num_tris = model.mesh.indices.len() / 3;
        let mut i_idx = 0;
        while i_idx < num_tris {
            let v_idx1 = model.mesh.indices[3 * i_idx] as usize;
            let v_idx2 = model.mesh.indices[(3 * i_idx) + 1] as usize;
            let v_idx3 = model.mesh.indices[(3 * i_idx) + 2] as usize;

            let v1_x = model.mesh.positions[3 * v_idx1];
            let v1_y = model.mesh.positions[(3 * v_idx1) + 1];
            let v1_z = model.mesh.positions[(3 * v_idx1) + 2];

            let v2_x = model.mesh.positions[3 * v_idx2];
            let v2_y = model.mesh.positions[(3 * v_idx2) + 1];
            let v2_z = model.mesh.positions[(3 * v_idx2) + 2];

            let v3_x = model.mesh.positions[3 * v_idx3];
            let v3_y = model.mesh.positions[(3 * v_idx3) + 1];
            let v3_z = model.mesh.positions[(3 * v_idx3) + 2];

            let v1 = Vec3::new(v1_x, v1_y, v1_z);
            let v2 = Vec3::new(v2_x, v2_y, v2_z);
            let v3 = Vec3::new(v3_x, v3_y, v3_z);

            let edge1 = v2 - v1;
            let edge2 = v3 - v1;
            let normal = Vec3::unit_vector(edge1.cross(edge2));
            let material = Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.5, 0.5, 0.5))));
            let tris = Triangle::new(v1, v2, v3, normal, Box::new(material));
            world.push(Box::new(tris));
            i_idx = i_idx + 1;
        }
    }

    let bvh_tree: Box<Hitable + Sync> = Box::new(BvhNode::new(world));
    //Setup camera
    let lookfrom = 3.0 * Vec3::new(-2.26788425, 0.320256859, 1.83503199);
    let lookat = Vec3::new(-1.33643341, 0.320256859, 1.47116470);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, image_width as f32 / image_height as f32, aperture, focus_dist);

    //Generate image
    let mut data = Vec::new();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut samples = vec![Vec3::zero_vector(); samples_per_pixel];

            samples.par_iter_mut().for_each( |sample| {
                let u = (x as f32 + rand::random::<f32>()) / image_width as f32;
                let v = (y as f32 + rand::random::<f32>()) / image_height as f32;

                let r = camera.get_ray(u, v);
                *sample = color(&r, &bvh_tree, 0);
            });

            let mut avg_color = Vec3::zero_vector();

            samples.iter().for_each( |sample| {
                avg_color = avg_color + *sample;
            });

            avg_color = avg_color / samples_per_pixel as f32;

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
        print!("{} / {} scanlines rendered \r", (image_height - y), image_height)
    }

    //Save end time
    let end_time = std::time::Instant::now();

    let render_duration = end_time.duration_since(start_time);
    let render_time_sec = render_duration.as_secs();
    let render_time_ms = render_duration.subsec_millis();

    println!("Render took {}.{} seconds", render_time_sec, render_time_ms);

    //Store image to file
    let path = Path::new("output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(&data).unwrap();

    println!("Done");
}

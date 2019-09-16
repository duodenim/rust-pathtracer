#![allow(dead_code)]
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

extern crate png;

extern crate rand;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
use hitable::Hitable;
use hitable::BvhNode;

mod sphere;
use sphere::Sphere;

mod triangle;
use triangle::Triangle;

mod material;
use material::{DiffuseLight, Lambertian};

mod camera;
use camera::Camera;

mod texture;
use texture::ConstantTexture;

mod aabb;

extern crate rayon;
use rayon::prelude::*;

extern crate obj;
use obj::Obj;

extern crate clap;
use clap::{Arg, App};

fn triangulate(vertices: Vec<Vec3>) -> Vec<Box<dyn Hitable + Sync>> {
    assert!(vertices.len() >= 3, "Input face must have at least 3 vertices!");
    let mut output: Vec<Box<dyn Hitable + Sync>> = Vec::new();

    //Trivial case: exactly 3 vertices are passed in
    if vertices.len() == 3 {
        let edge1 = vertices[1] - vertices[0];
        let edge2 = vertices[2] - vertices[0];
        let normal = Vec3::unit_vector(edge1.cross(edge2));
        let material = Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))));
        output.push(Box::new(Triangle::new(vertices[0], vertices[1], vertices[2], normal, Box::new(material))));
    } else { //Non trivial case - parse vertices as triangle fan
        let common_idx = 0;
        let mut first_idx = 1;
        let mut second_idx = 2;

        let common_v = vertices[common_idx];

        while second_idx < vertices.len() {
            let v1 = vertices[first_idx];
            let v2 = vertices[second_idx];
            let edge1 = v1 - common_v;
            let edge2 = v2 - common_v;
            let normal = Vec3::unit_vector(edge1.cross(edge2));
            let material = Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))));
            output.push(Box::new(Triangle::new(common_v, v1, v2, normal, Box::new(material))));
            first_idx = first_idx + 1;
            second_idx = second_idx + 1;
        }
    }
    output
}

fn color(r : &Ray, world: &Box<dyn Hitable + Sync>, depth: u32) -> Vec3 {
    let hit_rec = world.hit(0.001, 50.0, r);
    if hit_rec.is_some() {
        let hit_rec = hit_rec.unwrap();
        let material = hit_rec.material;
        let normal = hit_rec.normal;
        let point = hit_rec.p;
        let t = hit_rec.t;
        let scatter_rec = material.scatter(r, t, point, normal);
        let emitted = material.emitted(0.0, 0.0, &point);
        if scatter_rec.is_some() && depth < 50 {
            let scatter_rec = scatter_rec.unwrap();
            return emitted + scatter_rec.attenuation * color(&scatter_rec.scattered, world, depth + 1);
        } else {
            return emitted;
        }
    }
    Vec3::new(0.0, 0.0, 0.0)
    //let unit_direction = Vec3::unit_vector(r.direction());
    //let t = 0.5 * (unit_direction.y() + 1.0);
    //(1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    //Setup args
    let matches = App::new("Pathtracer")
                        .arg(Arg::with_name("INPUT")
                                    .required(true))
                        .arg(Arg::with_name("samples_per_pixel")
                                    .short("s")
                                    .long("spp")
                                    .help("Number of samples per pixel")
                                    .takes_value(true))
                        .arg(Arg::with_name("width")
                                    .short("w")
                                    .long("width")
                                    .help("Rendered image width")
                                    .takes_value(true))
                        .arg(Arg::with_name("height")
                                    .short("h")
                                    .long("height")
                                    .help("Rendered image height")
                                    .takes_value(true))
                        .arg(Arg::with_name("output")
                                    .short("o")
                                    .long("output")
                                    .help("Output image path")
                                    .takes_value(true))
                        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let samples_per_pixel = matches.value_of("samples_per_pixel").unwrap_or("100");
    let image_width = matches.value_of("width").unwrap_or("480");
    let image_height = matches.value_of("height").unwrap_or("270");
    let output_filename = matches.value_of("output").unwrap_or("output.png");

    let samples_per_pixel = samples_per_pixel.parse::<usize>().unwrap();
    let image_width = image_width.parse::<u32>().unwrap();
    let image_height = image_height.parse::<u32>().unwrap();

    println!("Generating a {}x{}@{}spp render of {}, saving to {}", image_width, image_height, samples_per_pixel, filename, output_filename);

    //Generate world
    let start_time = std::time::Instant::now();
    let mut world: Vec<Box<dyn Hitable + Sync>> = Vec::new();

    let obj_file = Obj::<obj::SimplePolygon>::load(Path::new(filename)).unwrap();
    for object in obj_file.objects.iter() {
        for group in object.groups.iter() {
            for polygon in group.polys.iter() {
                let mut vertices: Vec<Vec3> = Vec::new();
                for vertex in polygon.iter() {
                    let index = vertex.0;
                    let position = obj_file.position[index];
                    vertices.push(Vec3::new(position[0], position[1], position[2]));
                }
                world.append(&mut triangulate(vertices));
            }
        }
    }

    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 2.0), 0.5, Box::new(DiffuseLight::new(Box::new(ConstantTexture::new(Vec3::new(2.0, 2.0, 2.0))))))));
    let end_time = std::time::Instant::now();

    let duration = end_time.duration_since(start_time);
    let time_sec = duration.as_secs();
    let time_ms = duration.subsec_millis();

    println!("Scene loading took {}.{} seconds", time_sec, time_ms);

    let start_time = std::time::Instant::now();

    let bvh_tree: Box<dyn Hitable + Sync> = Box::new(BvhNode::new(world));

    let end_time = std::time::Instant::now();

    let duration = end_time.duration_since(start_time);
    let time_sec = duration.as_secs();
    let time_ms = duration.subsec_millis();

    println!("BVH generation took {}.{} seconds", time_sec, time_ms);

    //Setup camera
    let lookfrom = 6.0 * Vec3::new(-2.26788425, 0.320256859, 1.83503199);
    let lookat = Vec3::new(-1.33643341, 0.320256859, 1.47116470);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, image_width as f32 / image_height as f32, aperture, focus_dist);

    //Generate image
    let mut data = Vec::new();

    //Save start time
    let start_time = std::time::Instant::now();

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
            avg_color = avg_color.clamp(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
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
    let path = Path::new(output_filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(&data).unwrap();

    println!("Done");
}

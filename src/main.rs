use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::env;

extern crate png;
use png::HasParameters;

extern crate rand;

extern crate gif;
use gif::Parameter;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;
use sphere::Hit;

mod material;
use material::Lambertian;
use material::Metal;

mod camera;
use camera::Camera;

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
    let image_width: u16= 960;
    let image_height: u16 = 540;
    let samples_per_pixel = 100;
    let num_frames = 60;

    let args: Vec<String> = env::args().collect();

    let mut filename = "image.png";
    if args.len() < 2 {
        println!("Saving to image.png....");
    } else {
        filename = &args[1];
        println!("Saving to {}....", filename);
    }

    //Generate world
    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))));
    world.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0))));
    world.push(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3))));

    //Setup camera
    let mut output_gif = Vec::new();

    let step = (360.0 as f32).to_radians() / num_frames as f32;
    for frame_num in 0..num_frames {
        let this_step = frame_num as f32 * step;
        let camera = Camera::new(Vec3::new(-2.0 * this_step.sin(), 2.0, 2.0 * this_step.cos()), Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, image_width as f32 / image_height as f32);

        //Generate image
        let mut data = Vec::new();

        for y in (0..image_height).rev() {
            for x in 0..image_width {
                let mut avg_color = Vec3::zero_vector();

                for _ in 0..samples_per_pixel {
                    let u = (x as f32 + rand::random::<f32>()) / image_width as f32;
                    let v = (y as f32 + rand::random::<f32>()) / image_height as f32;

                    //let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

                    let r = camera.get_ray(u, v);
                    //println!("Ray is: {:?}", r);
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
        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, image_width.into(), image_height.into());
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();


        writer.write_image_data(&data).unwrap();

        let frame = gif::Frame::from_rgba(image_width, image_height, &mut data);

        println!("Frame delay is {}", frame.delay);

        output_gif.push(gif::Frame::from_rgba(image_width, image_height, &mut data));
    }

    let mut gif_file = File::create("output.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut gif_file, image_width, image_height, &[]).unwrap();
    gif::Repeat::Infinite.set_param(&mut encoder).unwrap();

    for frame in output_gif.iter() {
        encoder.write_frame(&frame).unwrap();
    }


    println!("Done");
}

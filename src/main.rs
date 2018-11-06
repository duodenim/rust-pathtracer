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

mod sphere;
use sphere::Sphere;
use sphere::Hit;

mod material;
use material::Lambertian;
use material::Metal;
use material::Dielectric;

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
    let image_width = 480;
    let image_height = 270;
    let samples_per_pixel = 100;

    let args: Vec<String> = env::args().collect();

    let mut filename = "image.png";
    if args.len() < 2 {
        println!("Saving to image.png....");
    } else {
        filename = &args[1];
        println!("Saving to {}....", filename);
    }

    //Save start time
    let start_time = std::time::Instant::now();

    //Generate world as seen in Chapter 12
    let mut world = Vec::new();
    world.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(), 0.2, b as f32 + 0.9 * rand::random::<f32>());
            if (center-Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Sphere::new(center, 0.2, Box::new(Lambertian::new(Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>())))));
                } else if choose_mat < 0.95 {
                    world.push(Sphere::new(center, 0.2, Box::new(Metal::new(Vec3::new(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>()), 0.5 * rand::random::<f32>()))));
                } else {
                    world.push(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5))));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))));

    //Setup camera
    let lookfrom = Vec3::new(12.0, 2.0, 2.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.0;
    let camera = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, image_width as f32 / image_height as f32, aperture, focus_dist);

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

    //Save end time
    let end_time = std::time::Instant::now();

    let render_duration = end_time.duration_since(start_time);
    let render_time_sec = render_duration.as_secs();
    let render_time_ms = render_duration.subsec_millis();

    println!("Render took {}.{} seconds", render_time_sec, render_time_ms);

    //Store image to file
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(&data).unwrap();

    println!("Done");
}

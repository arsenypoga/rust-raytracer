use ::raytracer::render::canvas::Canvas;
use ::raytracer::units::color::{QuantColor, RED, WHITE};
use ::raytracer::units::tuple::{Point, Tuple, Vector};
use ::raytracer::units::{Intersection, Ray, Sphere};
use ::raytracer::world::{tick, Environment, Material, PointLight, Projectile};
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "projectile" => simulate_projectile(),
        "canvas" => simulate_projectile_on_canvas(),
        "shadow" => draw_shadow(400),
        _ => println!("Command not recognized!"),
    }
    let duration = start.elapsed();
    println!("Execution took: {:?}", duration);
}

fn simulate_projectile() {
    let mut p = Projectile {
        position: Point::new(0, 1, 0),
        velocity: Vector::new(1, 1, 0).normalize(),
    };
    let e = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let mut counter = 0;
    while p.position.y >= 0. {
        p = tick(&e, &p);
        println!("Projectile Position: {:?}", p.position);
        counter += 1;
    }
    println!("Took {} ticks.", counter);
}

fn simulate_projectile_on_canvas() {
    let mut p = Projectile {
        position: Point::new(0, 1, 0),
        velocity: Vector::new(1., 1.8, 0.).normalize() * 11.25,
    };
    let e = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let mut canvas = Canvas::new(900, 550);

    while p.position.y >= 0. && p.position.x >= 0. {
        canvas.write_pixel(
            p.position.x as usize,
            canvas.height - p.position.y as usize,
            RED,
        );
        p = tick(&e, &p);
    }

    canvas.write_png("images/projectile_on_canvas.png");
}

fn draw_shadow(size: usize) {
    const WALL_SIZE: usize = 7;
    let ray_origin = Point::new(0, 0, -5);
    let wall_z = 10.0;
    let pixel_size = WALL_SIZE as f64 / size as f64;
    let half = WALL_SIZE as f64 / 2.0;

    let mut canvas = Canvas::new(size, size);
    // let color = QuantColor::new(255, 0, 0);

    // let mut shape = Sphere::new();
    // shape.material = Material::default();
    // shape.material.color = QuantColor::new(255, 42, 255);

    let shape = Sphere {
        material: Material {
            color: QuantColor::new(255, 42, 255),
            ..Material::default()
        },
        ..Sphere::default()
    };

    let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
    for y in 0..size {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..size {
            // println!("x:{:}, y:{:}", x, y);
            let world_x = -half + pixel_size * (x as f64);
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());

            let xs = r.intersect(&shape);
            if xs.len() != 0 {
                let hit = Intersection::hit(xs).unwrap();
                let hit_point = r.position(hit.t);
                let hit_normal = hit.object.normal(hit_point);
                let eyev = -r.direction;
                let color = hit
                    .object
                    .material
                    .lightning(light, hit_point, eyev, hit_normal)
                    .clamp();
                canvas.write_pixel(x, y, color);
            }
        }
    }
    canvas.write_png("images/shadow.png");
}

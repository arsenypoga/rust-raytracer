use ::raytracer::render::{Camera, Canvas, World};
use ::raytracer::units::color::{QuantColor, RED, WHITE};
use ::raytracer::units::objects::{ObjectType, Shape};
use ::raytracer::units::tuple::{Point, Tuple, Vector};
use ::raytracer::units::{Intersection, Matrix, Ray, Transformable};
use ::raytracer::world::patterns::{Pattern, PatternType};
use ::raytracer::world::{tick, Environment, Material, PointLight, Projectile};
use std::env;
use std::f64::consts;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let (hsize, vsize, size): (usize, usize, usize) = if cfg!(debug_assertions) {
        (100, 100, 100)
    } else {
        (1000, 1000, 1000)
    };

    match args[1].as_ref() {
        "projectile" => simulate_projectile(),
        "canvas" => simulate_projectile_on_canvas(),
        "shadow" => draw_shadow(size),
        "render_sphere_world" => render_sphere_only_world(hsize, vsize),
        "render_plane_world" => render_plane_world(hsize, vsize),
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

    let shape = Shape {
        material: Material {
            color: QuantColor::new(255, 42, 255),
            ..Material::default()
        },
        ..Shape::default()
    };

    let light = PointLight::new(Point::new(-10, 10, -10), WHITE);
    for y in 0..size {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..size {
            // println!("x:{:}, y:{:}", x, y);
            let world_x = -half + pixel_size * (x as f64);
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());

            let xs = shape.intersect(r);
            if !xs.is_empty() {
                let hit = Intersection::hit(xs).unwrap();
                let hit_point = r.position(hit.t);
                let hit_normal = hit.object.normal(hit_point);
                let eyev = -r.direction;
                let color = hit
                    .object
                    .lightning(light, hit_point, eyev, hit_normal, false)
                    .clamp();
                canvas.write_pixel(x, y, color);
            }
        }
    }
    canvas.write_png("images/shadow.png");
}

fn render_sphere_only_world(hsize: usize, vsize: usize) {
    let mut floor = Shape::default();
    floor.transformation_matrix = Matrix::scale(10., 0.01, 10.);
    floor.material.color = QuantColor::new(255, 240, 240);
    floor.material.specular = 0.;

    let mut left_wall = Shape::default();
    left_wall.transformation_matrix = Matrix::translate(0, 0, 5)
        * Matrix::rotate_y(-consts::FRAC_PI_4)
        * Matrix::rotate_x(consts::FRAC_PI_2)
        * Matrix::scale(10., 0.01, 10.);
    left_wall.material = floor.material;

    let mut right_wall = Shape::default();
    right_wall.transformation_matrix = Matrix::translate(0, 0, 5)
        * Matrix::rotate_y(consts::FRAC_PI_4)
        * Matrix::rotate_x(consts::FRAC_PI_2)
        * Matrix::scale(10., 0.01, 10.);
    right_wall.material = floor.material;

    let mut middle = Shape::default();
    middle.transformation_matrix = Matrix::translate(-0.5, 1., 0.5);
    middle.material.color = QuantColor::new(125, 255, 10);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Shape::default();
    right.transformation_matrix = Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5);
    right.material.color = QuantColor::new(10, 255, 125);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Shape::default();
    left.transformation_matrix =
        Matrix::translate(-1.5, 0.5, -0.75) * Matrix::scale(0.33, 0.33, 0.33);
    left.material.color = QuantColor::new(255, 25, 10);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let camera = Camera::new(hsize, vsize, consts::FRAC_PI_3).transform(Matrix::view_transform(
        Point::new(0., 1.5, -5.),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    let mut world = World::new();
    world.light = Some(PointLight::new(Point::new(-10, 10, -10), WHITE));
    world.objects = vec![left, right, middle, left_wall, right_wall, floor];

    let canvas = camera.render(world);

    canvas.write_png("./images/render_world.png");
}

fn render_plane_world(hsize: usize, vsize: usize) {
    println!("Rendering Plane World");
    let floor = Shape::new(ObjectType::Plane)
        .set_material(
            Material::default()
                .set_pattern(Some(
                    Pattern::new(PatternType::Checkers(
                        QuantColor::new(0, 0, 125),
                        QuantColor::new(100, 40, 0),
                    ))
                    .translate(10, 0, 0)
                    .scale(0.1, 0.1, 0.1),
                ))
                .set_color(QuantColor::new(255, 240, 240))
                .set_specular(0.),
        )
        .scale(10., 0.01, 10.);

    let wall = Shape::new(ObjectType::Plane)
        .set_material(
            Material::default().set_pattern(Some(Pattern::default().set_pattern_type(
                PatternType::Stripe(QuantColor::new(150, 0, 0), QuantColor::new(0, 120, 30)),
            ))),
        )
        .rotate_x(consts::FRAC_PI_2)
        .translate(0, 3, 0);

    let middle = Shape::default()
        .set_material(
            Material::default()
                .set_pattern(Some(
                    Pattern::default()
                        .set_pattern_type(PatternType::Stripe(
                            QuantColor::new(0, 230, 120),
                            QuantColor::new(25, 60, 80),
                        ))
                        .scale(0.2, 0.2, 0.2)
                        // .rotate_x(consts::FRAC_PI_2)
                        // .rotate_y(consts::FRAC_PI_2)
                        .rotate_z(consts::FRAC_PI_2),
                ))
                .set_color(QuantColor::new(125, 255, 10))
                .set_diffuse(0.7)
                .set_specular(0.3),
        )
        .translate(-0.5, 1., 0.5);

    let right = Shape::default()
        .transform(Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5))
        .set_material(
            Material::default()
                .set_color(QuantColor::new(10, 255, 125))
                .set_pattern(Some(
                    Pattern::new(PatternType::Gradient(
                        QuantColor::new(100, 0, 0),
                        QuantColor::new(0, 100, 0),
                    ))
                    .rotate_x(consts::FRAC_PI_2),
                ))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let left = Shape::default()
        .transform(Matrix::translate(-1.5, 0.5, -0.75) * Matrix::scale(0.33, 0.33, 0.33))
        .set_material(
            Material::default()
                .set_color(QuantColor::new(255, 25, 10))
                .set_diffuse(0.7)
                .set_specular(0.3),
        );

    let camera = Camera::new(hsize, vsize, consts::FRAC_PI_3).transform(Matrix::view_transform(
        Point::new(-3., 1.5, -5.),
        Point::new(0, 1, 0),
        Vector::new(0, 1, 0),
    ));

    let world = World::new()
        .set_light(Some(PointLight::new(Point::new(-10, 10, -10), WHITE)))
        .set_objects(vec![left, right, middle, floor, wall]);

    let canvas = camera.render(world);

    canvas.write_png("./images/render_plane_world.png");
}

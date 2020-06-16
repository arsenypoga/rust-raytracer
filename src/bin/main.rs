use ::raytracer::render::{Camera, Canvas, World};
use ::raytracer::units::color::{QuantColor, BLACK, RED, WHITE};
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
        "render_refract_scene" => render_reflect_scene(hsize, vsize),
        "ok" => refraction_render(hsize, vsize),
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
                    Pattern::new(PatternType::Checkers(WHITE, BLACK))
                        .translate(10, 0, 0)
                        .scale(0.1, 0.1, 0.1),
                ))
                .set_color(QuantColor::new(255, 240, 240))
                .set_specular(0.)
                .set_reflect(1.),
        )
        .scale(5, 5, 5);

    let center_wall = Shape::new(ObjectType::Plane)
        .set_material(
            Material::default()
                .set_pattern(Some(
                    Pattern::new(PatternType::Checkers(WHITE, BLACK))
                        .translate(10, 0, 0)
                        .scale(0.1, 0.1, 0.1),
                ))
                .set_color(QuantColor::new(255, 240, 240))
                .set_specular(0.),
        )
        .scale(5, 5, 5)
        .rotate_x(consts::FRAC_PI_2);

    let side_wall = Shape::new(ObjectType::Plane)
        .set_material(
            Material::default()
                .set_pattern(Some(
                    Pattern::new(PatternType::Checkers(WHITE, BLACK))
                        .translate(10, 0, 0)
                        .scale(0.1, 0.1, 0.1),
                ))
                .set_color(QuantColor::new(255, 240, 240))
                .set_specular(0.),
        )
        .scale(5, 5, 5)
        .rotate_x(consts::FRAC_PI_2)
        .rotate_z(consts::FRAC_PI_2);
    // .translate(0, 2, 0);

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
        .translate(3.7, 1., 3.7);

    let right = Shape::default()
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
        )
        .translate(3.5, 0.5, 5.5)
        .scale(0.5, 0.5, 0.5);

    let left = Shape::default()
        .set_material(
            Material::default()
                .set_color(QuantColor::new(255, 25, 10))
                .set_diffuse(0.7)
                .set_specular(0.3),
        )
        .translate(5.5, 0.5, 3.75)
        .scale(0.33, 0.33, 0.33);

    let camera = Camera::new(hsize, vsize, consts::FRAC_PI_3).transform(Matrix::view_transform(
        Point::new(7., 2.5, 7.),
        Point::new(0, 0, 0),
        Vector::new(0, 1, 0),
    ));

    let world = World::new()
        .set_light(Some(PointLight::new(Point::new(5, 5, 10), WHITE)))
        .set_objects(vec![left, right, middle, floor, center_wall, side_wall]);

    let canvas = camera.render(world);

    canvas.write_png("./images/render_plane_world.png");
}

fn render_reflect_scene(hsize: usize, vsize: usize) {
    let camera = Camera::new(hsize, vsize, consts::FRAC_PI_3).transform(Matrix::view_transform(
        Point::new(-2.6, 1.5, -3.9),
        Point::new(-0.6, 1., -0.8),
        Vector::new(0, 1, 0),
    ));

    let light = PointLight::new(Point::new(10, 10, 0), WHITE);

    let wall_material = Material::default()
        .set_pattern(Some(
            Pattern::new(PatternType::Stripe(
                QuantColor::new(114, 114, 114),
                QuantColor::new(140, 140, 140),
            ))
            .scale(0.25, 0.25, 0.25)
            .rotate_y(consts::FRAC_PI_2),
        ))
        .set_ambient(0.)
        .set_diffuse(0.4)
        .set_specular(0.)
        .set_reflect(0.3);

    let floor = Shape::new(ObjectType::Plane).set_material(
        Material::default()
            .set_pattern(Some(Pattern::new(PatternType::Checkers(
                QuantColor::new(89, 89, 89),
                QuantColor::new(166, 166, 166),
            ))))
            .set_specular(0.)
            .set_reflect(0.4),
    );

    let ceiling = Shape::new(ObjectType::Plane)
        .translate(0, 5, 0)
        .set_material(
            Material::default()
                .set_color(QuantColor::new(204, 204, 204))
                .set_ambient(0.3)
                .set_specular(0.),
        );

    let west_wall = Shape::new(ObjectType::Plane)
        .rotate_y(consts::FRAC_PI_2)
        .rotate_z(consts::FRAC_PI_2)
        .translate(-5, 0, 0)
        .set_material(wall_material);

    let east_wall = Shape::new(ObjectType::Plane)
        .rotate_y(consts::FRAC_PI_2)
        .rotate_z(consts::FRAC_PI_2)
        .translate(5, 0, 0)
        .set_material(wall_material);

    let north_wall = Shape::new(ObjectType::Plane)
        .rotate_x(consts::FRAC_PI_2)
        .translate(0, 0, 5)
        .set_material(wall_material);

    let south_wall = Shape::new(ObjectType::Plane)
        .rotate_x(consts::FRAC_PI_2)
        .translate(0, 0, -5)
        .set_material(wall_material);

    let world = World::new().set_light(Some(light)).set_objects(vec![
        floor, ceiling, west_wall, north_wall, south_wall, east_wall,
    ]);

    let result = camera.render(world);
    result.write_png("./images/render_refract_scene.png");
}

fn refraction_render(hsize: usize, vsize: usize) {
    let mut world = World::new();
    let light = Some(PointLight::new(Point::new(0, 10, 0), WHITE));
    world.light = light;

    let floor = Shape::new(ObjectType::Plane)
        .set_material(Material::default().set_pattern(Some(
            Pattern::new(PatternType::Checkers(BLACK, WHITE)).scale(-5, -5, -5),
        )))
        // .scale(5, 5, 5)
        .translate(-15, 0, 0)
        .rotate_z(consts::FRAC_PI_2);

    let big_sphere = Shape::new(ObjectType::Sphere)
        .set_material(
            Material::default()
                .set_transparency(1.)
                .set_refractive_index(1.5)
                .set_reflect(1.)
                .set_color(WHITE),
        )
        .scale(4, 4, 4);

    let small_sphere = Shape::new(ObjectType::Sphere).set_material(
        Material::default()
            .set_transparency(1.)
            .set_refractive_index(1.)
            .set_reflect(1.)
            .set_color(BLACK),
    );
    world.objects = vec![floor, big_sphere, small_sphere];

    let camera = Camera::new(hsize, vsize, consts::FRAC_PI_3).transform(Matrix::view_transform(
        Point::new(15, 0, 0),
        Point::new(0, 0, 0),
        Vector::new(0, 1, 0),
    ));

    let canvas = camera.render(world);
    canvas.write_png("./images/refractive_sphere.png");
}

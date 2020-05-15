use ::raytracer::units::tuple::{Point, Tuple, Vector};
use ::raytracer::world::{tick, Environment, Projectile};
use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    match args[1].as_ref() {
        "projectile" => simulate_projectile(),
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

use parser::generator::{generate_world, read_file};
use std::{env, time::Instant};

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    let data = read_file(args[1].clone()).unwrap();
    let (w, c) = generate_world(data);
    let canvas = c.render(w);
    canvas.write_png("./images/yaml_generated.png");

    let duration = start.elapsed();
    println!("Execution took: {:?}", duration);
}

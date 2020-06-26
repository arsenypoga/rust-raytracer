use crate::{
    types::{Action, TransformActions},
    Data,
};
use raytracer::{
    render::{Camera, World},
    units::{objects::Shape, Matrix, Transformable},
    world::{Material, PointLight},
};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Data, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let res = serde_yaml::from_reader(reader)?;
    Ok(res)
}

pub fn generate_world(data: Data) -> (World, Camera) {
    let mut camera: Option<Camera> = None;
    let mut light: Option<PointLight> = None;
    let mut objects = Vec::<Shape>::new();
    let mut definitions = HashMap::<String, Material>::new();

    let mut w = World::new();

    for action in data {
        match action {
            Action::AddCamera {
                width,
                height,
                field_of_view,
                from,
                to,
                up,
            } => {
                camera = {
                    Some(
                        Camera::new(height, width, field_of_view)
                            .transform(Matrix::view_transform(from, to, up)),
                    )
                }
            }
            Action::AddLight { at, intensity } => light = Some(PointLight::new(at, intensity)),
            Action::AddObject {
                object_type,
                material,
                transform,
            } => {
                let mut object = Shape::new(object_type);

                for transformation in transform {
                    object = match transformation {
                        TransformActions::Scale(n) => object.scale(n[0], n[1], n[2]),
                        TransformActions::Translate(n) => object.translate(n[0], n[1], n[2]),
                        TransformActions::RotateX(deg) => object.rotate_x(deg),
                        TransformActions::RotateY(deg) => object.rotate_y(deg),
                        TransformActions::RotateZ(deg) => object.rotate_z(deg),
                    }
                }

                object.material = *definitions.get(&material).expect("No Material");

                objects.push(object);
            }
            Action::DefineMaterial {
                name,
                color,
                ambient,
                diffuse,
                specular,
                shine,
                reflect,
                transparent,
                refractive_index,
            } => {
                definitions.insert(
                    name,
                    Material::new(color)
                        .set_specular(specular)
                        .set_ambient(ambient)
                        .set_diffuse(diffuse)
                        .set_shine(shine)
                        .set_reflect(reflect)
                        .set_transparency(transparent)
                        .set_refractive_index(refractive_index),
                );
            }
        }
    }

    w.light = light;
    w.objects = objects;
    (w, camera.expect("No camera!"))
}

#[cfg(test)]
mod test {
    // use super::*;
    #[test]
    fn read_file() {
        let f = super::read_file("./scene.yaml");
    }
}

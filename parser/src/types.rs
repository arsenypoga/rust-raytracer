use raytracer::{
    self,
    units::tuple::{Point, Vector},
    units::{
        color::{QuantColor, WHITE},
        objects::ObjectType,
    },
    world::patterns::PatternType,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Action {
    #[serde(rename = "add camera")]
    AddCamera {
        width: usize,
        height: usize,
        #[serde(rename = "field-of-view")]
        field_of_view: f64,
        #[serde(with = "PointDef")]
        from: Point,
        #[serde(with = "PointDef")]
        to: Point,
        #[serde(with = "VectorDef")]
        up: Vector,
    },

    #[serde(rename = "add light")]
    AddLight {
        #[serde(with = "PointDef")]
        at: Point,
        #[serde(with = "QuantColorDef")]
        intensity: QuantColor,
    },

    #[serde(rename = "add object")]
    AddObject {
        #[serde(rename = "type", with = "ObjectTypeDef")]
        object_type: ObjectType,
        material: String,
        transform: Vec<TransformActions>,
    },

    #[serde(rename = "define material")]
    DefineMaterial {
        name: String,
        #[serde(with = "QuantColorDef")]
        color: QuantColor,
        #[serde(default = "default_ambient")]
        ambient: f64,
        #[serde(default = "default_diffuse")]
        diffuse: f64,
        #[serde(default = "default_specular")]
        specular: f64,
        #[serde(default = "default_shine")]
        shine: f64,
        #[serde(default = "default_reflect")]
        reflect: f64,
        #[serde(default = "default_transparent")]
        transparent: f64,
        #[serde(default = "default_refractive_index")]
        refractive_index: f64,
    },
}

fn default_ambient() -> f64 {
    0.1
}

fn default_diffuse() -> f64 {
    0.9
}

fn default_specular() -> f64 {
    0.9
}

fn default_shine() -> f64 {
    200.
}

fn default_reflect() -> f64 {
    0.
}
fn default_transparent() -> f64 {
    0.
}

fn default_refractive_index() -> f64 {
    1.
}
#[derive(Debug, Deserialize)]
#[serde(remote = "ObjectType", rename_all = "lowercase")]
pub enum ObjectTypeDef {
    Sphere,
    Plane,
}
#[derive(Debug, Deserialize)]
#[serde(remote = "Point", rename_all = "lowercase", from = "[f64; 3]")]
pub struct PointDef {
    x: f64,
    y: f64,
    z: f64,
    #[serde(getter = "Point::get_w")]
    w: f64,
}

#[derive(Debug, Deserialize)]
#[serde(remote = "Vector", rename_all = "lowercase", from = "[f64; 3]")]
pub struct VectorDef {
    x: f64,
    y: f64,
    z: f64,
    #[serde(getter = "Vector::get_w")]
    w: f64,
}

impl From<[f64; 3]> for PointDef {
    fn from(a: [f64; 3]) -> Self {
        PointDef {
            x: a[0],
            y: a[1],
            z: a[2],
            w: 1.,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(remote = "QuantColor", rename_all = "lowercase", from = "[i64; 3]")]
pub struct QuantColorDef {
    r: i64,
    g: i64,
    b: i64,
}

impl From<[i64; 3]> for QuantColorDef {
    fn from(a: [i64; 3]) -> Self {
        QuantColorDef {
            r: a[0],
            g: a[1],
            b: a[2],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransformActions {
    Scale([f64; 3]),
    Translate([f64; 3]),
    #[serde(rename = "rotate-x")]
    RotateX(f64),
    #[serde(rename = "rotate-y")]
    RotateY(f64),
    #[serde(rename = "rotate-z")]
    RotateZ(f64),
}

pub type Data = Vec<Action>;

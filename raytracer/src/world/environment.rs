//! Environment simulation
use crate::units::tuple::{Point, Vector};

/// Projectile is a projectile
#[derive(Debug, Copy, Clone)]
pub struct Projectile {
    /// current position
    pub position: Point,
    /// current velocity
    pub velocity: Vector,
}
/// Environment is a environment that acts on a projectile
#[derive(Debug, Copy, Clone)]
pub struct Environment {
    /// how much movement is experiencing down
    pub gravity: Vector,
    /// how much movement is experiencing horizontally
    pub wind: Vector,
}

/// Returns new projectile position after a tick
///
/// # Arguments
///
/// `env` - Environment
/// `proj` - Projetile
///
pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

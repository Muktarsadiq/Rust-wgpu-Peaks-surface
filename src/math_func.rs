#![allow(dead_code)]
use cgmath::*;

pub fn peaks(x: f32, z: f32) -> [f32; 3] {
    let y = 3.0 * (1.0 - x) * (1.0 - x) * (-(x * x) - (z + 1.0) * (z + 1.0)).exp()
        - 10.0 * (x / 5.0 - x * x * x - z * z * z * z * z) * (-x * x - z * z).exp()
        - 1.0 / 3.0 * (-(x + 1.0) * (x + 1.0) - z * z).exp();
    [x, y, z]
}

pub fn sinc(x: f32, z: f32) -> [f32; 3] {
    let r = (x * x + z * z).sqrt();
    let y = if r == 0.0 { 1.0 } else { r.sin() / r };
    [x, y, z]
}

pub fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> [f32; 3] {
    let x = (r_torus + r_tube * v.cos()) * u.cos();
    let y = r_tube * v.sin();
    let z = -(r_torus + r_tube * v.cos()) * u.sin();
    [x, y, z]
}

pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
    let x = r * theta.sin() * phi.cos();
    let y = r * theta.cos();
    let z = -r * theta.sin() * phi.sin();
    [x, y, z]
}

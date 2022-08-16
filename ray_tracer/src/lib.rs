#[macro_use]
extern crate lazy_static;

pub use vec3::*;
pub use image::*;
pub use renderer::*;
pub use scene::*;
pub use camera::*;
pub use shape::*;
pub use material::*;
pub use texture::*;
pub use pool::WorkerPool;

pub mod vec3;
pub mod ray;
pub mod utils;
pub mod image;
pub mod renderer;
pub mod scene;
pub mod camera;
pub mod shape;
pub mod aabb;
pub mod collisions;
pub mod material;
pub mod texture;
pub mod perlin;
pub mod pool;



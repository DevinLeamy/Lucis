use crate::material::{Material, MaterialType};
use crate::shape::{ShapeType};

#[readonly::make]
#[derive(Copy, Clone)]
pub struct Element {
    pub material: MaterialType, 
    pub shape: ShapeType,
}

#[readonly::make]
pub struct Scene {
    pub objects: Vec<Element> 
}

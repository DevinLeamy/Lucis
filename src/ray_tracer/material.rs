use crate::common::*;

pub struct Scatter {
    pub ray: Ray,
    pub texture: Rc<Box<dyn Texture>>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub fn make_shared_material<T>(material: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(material))
}

use crate::core::*;

struct Pallet;

impl Pallet {
    pub const RED: Color = Color::new(1f64, 0f64, 0f64);
    pub const GREEN: Color = Color::new(0f64, 1f64, 0f64);
    pub const BLUE: Color = Color::new(0f64, 0f64, 1f64);
    pub const BLACK: Color = Color::new(0f64, 0f64, 0f64);
    pub const WHITE: Color = Color::new(1f64, 1f64, 1f64);
    pub const C1: Color = Color::new(0.2f64, 0.9f64, 0.7f64);
    pub const C2: Color = Color::new(0.7f64, 0.7f64, 0.2f64);
    pub const C3: Color = Color::new(0.7f64, 0.9f64, 0.2f64);
}

#[macro_export]
macro_rules! shared_ptr {
    ( $x:expr ) => {
        {
            Rc::new(Box::new($x))
        }
    };
}

#[macro_export]
macro_rules! shared_mptr {
    ( $x:expr ) => {
        {
            Rc::new(RefCell::new(Box::new($x)))
        }
    };
}

pub fn texture_scene() -> HittableList {
    let checker = CheckeredTexture::from_colors(
        &Color::new(0.5f64, 1f64, 0.5f64),
        &Color::new(0.0f64, 0.5f64, 0.5f64),
    );

    let mut world = HittableList::default();

    let ground_material: Rc<Box<dyn Material>> = Rc::new(Box::new(
        Lambertian::from_texture(Rc::new(Box::new(checker))),
    ));

    world.add(Rc::new(RefCell::new(Box::new(RSphere::from_sphere(
        Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0),
        ground_material,
    )))));

    world
}

pub fn single_sphere() -> HittableList {
    let mut world = HittableList::default();

    let gnd: Rc<Box<dyn Material>> = shared_ptr!(Lambertian::from_color(&Pallet::RED));

    world.add(shared_mptr!(RSphere::from_sphere(Sphere::new(Point::ZEROS(), 10.0), gnd)));

    world
}

pub fn simple_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker: Rc<Box<dyn Texture>> = shared_ptr!(CheckeredTexture::from_colors(
        &Color::new(0.2f64, 0.8f64, 0.2f64),
        &Color::new(0.7f64, 0.2f64, 0.9f64),
    ));

    let gnd: Rc<Box<dyn Material>> = shared_ptr!(Lambertian::from_texture(checker));

    world.add(shared_mptr!(RSphere::from_sphere(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0), gnd)));

    let m1: Rc<Box<dyn Material>> = shared_ptr!(Dielectric::new(1.5));
    let m2: Rc<Box<dyn Material>> = shared_ptr!(Lambertian::new(Pallet::C1)); 
    let m3: Rc<Box<dyn Material>> = shared_ptr!(Metal::new(Pallet::C2, 0.0)); 

    world.add(shared_mptr!(RSphere::from_sphere(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0), m1)));
    world.add(shared_mptr!(RSphere::from_sphere(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0), m2)));
    world.add(shared_mptr!(RSphere::from_sphere(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0), m3)));

    world
}

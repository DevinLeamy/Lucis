use crate::core::*;

// pub fn simple_scene() -> HittableList {
//     let mut world = HittableList::default();

//     let checker = CheckeredTexture::from_colors(
//         &Color::new(0.2f64, 0.8f64, 0.2f64),
//         &Color::new(0.7f64, 0.2f64, 0.9f64),
//     );

//     let ground = SharedMut::new<dyn Material>(Lambertian::from_texture(Rc::new(Box::new(checker))));

//     let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(
//         Lambertian::from_texture(Rc::new(Box::new(checker))),
//     ));
//     world.add(make_shared_hittable(Box::new(Sphere::new(
//         Point::new(0.0, -1000.0, 0.0),
//         1000.0,
//         ground_material,
//     ))));

//     let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
//     let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
//         0.4, 0.2, 0.1,
//     ))));
//     let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
//         Color::new(0.7, 0.6, 0.5),
//         0.0,
//     )));

//     world.add(make_shared_hittable(Box::new(Sphere::new(
//         Point::new(0.0, 1.0, 0.0),
//         1.0,
//         m1,
//     ))));
//     world.add(make_shared_hittable(Box::new(Sphere::new(
//         Point::new(-4.0, 1.0, 0.0),
//         1.0,
//         m2,
//     ))));
//     world.add(make_shared_hittable(Box::new(Sphere::new(
//         Point::new(4.0, 1.0, 0.0),
//         1.0,
//         m3,
//     ))));

//     // world.add(make_shared_hittable(Box::new(Triangle::new(
//     //     Point::new(-5.0, 1.0, 0.0),
//     //     Point::new(3.0, 0.0, 0.0),
//     //     Point::new(2.0, 3.0, 0.0),
//     // ))));

//     world
// }

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
        Sphere::new(Point::new(0.0, 0.0, 0.0), 10.0),
        ground_material,
    )))));

    world
}

pub fn single_sphere() -> HittableList {
    let mut world = HittableList::default();

    let ground_material: Rc<Box<dyn Material>> = Rc::new(Box::new(
        Lambertian::from_color(&Color::new(0.5, 0.2, 0.9)),
    ));

    world.add(Rc::new(RefCell::new(Box::new(RSphere::from_sphere(
        Sphere::new(Point::new(0.0, 0.0, 0.0), 10.0),
        ground_material,
    )))));

    world
}

use rand::{prelude::StdRng, SeedableRng};

use crate::{cuboid::Triangle, *};

pub fn complex_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(
        Color::new(0.5, 0.5, 0.5),
    )));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for i in -11..11 {
        for j in -11..11 {
            let mat = random_float();
            let origin = Point::new(
                i as f64 + 0.9 * random_float(),
                0.2,
                j as f64 * random_float(),
            );

            if (origin - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if mat < 0.8 {
                    let albedo = Vec3::RAND() * Vec3::RAND();
                    make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(albedo)))
                } else if mat < 0.95 {
                    let albedo = Color::RAND_RANGE(0.5, 1.0);
                    let fuzz = random_float_in_range(0.0, 0.5);

                    make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(albedo, fuzz)))
                } else {
                    make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)))
                };

                world.add(make_shared_hittable(Box::new(Sphere::new(
                    origin, 0.2, material,
                ))))
            }
        }
    }

    let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
    let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));

    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m1,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m2,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m3,
    ))));

    world
}

pub fn complex_not_random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(
        Color::new(0.5, 0.5, 0.5),
    )));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for i in -11..11 {
        for j in -11..11 {
            let mat = (i + j) % 5;
            let origin = Point::new(i as f64 + 0.9, 0.2, j as f64 * 0.5);

            if (origin - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if mat < 3 {
                    let albedo = Color::new(0.2, 0.9, 1.0);
                    make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(albedo)))
                } else if mat == 3 {
                    let albedo = Color::new(0.7, 0.5, 1.0);
                    let fuzz = 0.3;

                    make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(albedo, fuzz)))
                } else {
                    make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)))
                };

                world.add(make_shared_hittable(Box::new(Sphere::new(
                    origin, 0.2, material,
                ))))
            }
        }
    }

    let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
    let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));

    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m1,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m2,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m3,
    ))));

    world
}

pub fn simple_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(
        Color::new(0.5, 0.5, 0.5),
    )));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
    let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));

    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m1,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m2,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m3,
    ))));

    world.add(make_shared_hittable(Box::new(Triangle::new(
        Point::new(-5.0, 1.0, 0.0),
        Point::new(3.0, 0.0, 0.0),
        Point::new(2.0, 3.0, 0.0),
    ))));

    world
}

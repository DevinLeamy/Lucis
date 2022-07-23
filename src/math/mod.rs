pub use vec3::*;

pub mod vec3;

pub fn clamp(min: f64, max: f64, value: f64) -> f64 {
    f64::min(max, f64::max(min, value))
}

pub fn sample_unit_sphere() -> Vec3 {
    loop {
        let sample = Vec3::RAND_RANGE(-1.0, 1.0);

        if sample.length_squared() < 1.0 {
            // sample is inside unit sphere
            return sample;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::normalized(sample_unit_sphere())
}

pub fn sample_hemisphere(normal: &Vec3) -> Vec3 {
    let unit_sphere_sample = sample_unit_sphere();

    if Vec3::dot(&unit_sphere_sample, normal) > 0.0 {
        // vectors point in the "same" direction
        unit_sphere_sample
    } else {
        // vectors point in "opposite" directions
        -unit_sphere_sample
    }
}

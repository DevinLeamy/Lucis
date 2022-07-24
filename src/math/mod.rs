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

pub fn reflect(incident: &Vec3, surface_normal: &Vec3) -> Vec3 {
    /*
    We take the incident vector, v, and compute the
    projection of v onto the surface normal, n. We then
    negate this value because v and n point in opposite directions.
    This gives us the "amount" of v that points in the
    direction of the normal, b. We then remove 2 * b from v
    effectively reversing the component of v that projects onto n.
    */
    *incident - *surface_normal * (Vec3::dot(incident, surface_normal) * 2.0)
}

// CLEAN: too many dereferences and references
pub fn refract(incident: &Vec3, surface_normal: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(
        Vec3::dot(&(-(*incident)), surface_normal) / (incident.length() * surface_normal.length()),
        1.0,
    );
    let refracted_perpendicular = (*incident + *surface_normal * cos_theta) * etai_over_etat;
    let refracted_parallel = *surface_normal
        * -(1.0 - refracted_perpendicular.length_squared())
            .abs()
            .sqrt();

    refracted_perpendicular + refracted_parallel
}

use rand::{thread_rng, Rng};
use crate::vec3::Vec3;

pub fn random_float() -> f64 {
    thread_rng().gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    random_float() * (max - min) + min
}

pub fn u32_random_range(min: u32, max: u32) -> u32 {
    (random_float() * ((max - min) + min) as f64) as u32
}

pub fn sample_unit_sphere() -> Vec3 {
    loop {
        let sample = Vec3::new(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        );

        if sample.length_squared() < 1.0 {
            // sample is inside unit sphere
            return sample;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::normalized(sample_unit_sphere())
}

pub fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    /*
    We take the incident vector, v, and compute the
    projection of v onto the surface normal, n. We then
    negate this value because v and n point in opposite directions.
    This gives us the "amount" of v that points in the
    direction of the normal, b. We then remove 2 * b from v
    effectively reversing the component of v that projects onto n.
    */
    incident - normal * (Vec3::dot(incident, normal) * 2.0)
}

pub fn refract(incident: Vec3, normal: Vec3, ref_ratio: f64) -> Vec3 {
    let cos = f64::min(Vec3::dot(-incident, normal), 1.0);
    // compute the perpendicular and parallel components 
    let ref_perp = (incident + normal * cos) * ref_ratio;
    let ref_par = normal * -(1.0 - ref_perp.length_squared()).abs().sqrt();

    ref_perp + ref_par 

}

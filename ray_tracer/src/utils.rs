use rand::{thread_rng, Rng};
use serde::{Serialize, Deserialize};
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

pub fn lerp(t0: f64, t1: f64, w: f64) -> f64 {
    t0 + (t1 - t0) * w
}

/// https://en.wikipedia.org/wiki/Trilinear_interpolation
pub fn t_lerp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    // x
    let c00 = lerp(c[0][0][0], c[1][0][0], u);
    let c01 = lerp(c[0][0][1], c[1][0][1], u);
    let c10 = lerp(c[0][1][0], c[1][1][0], u);
    let c11 = lerp(c[0][1][1], c[1][1][1], u);

    // y
    let c0 = lerp(c00, c10, v);
    let c1 = lerp(c01, c11, v);

    // z
    let c = lerp(c0, c1, w);

    c
}

pub fn perlin_t_lerp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0f64;

    // Hermitian Smoothing
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);

                acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                       (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                       (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) *
                       Vec3::dot(c[i][j][k], weight);
            }
        }
    }

    acc
}

use crate::vec3::Vec3;
use crate::utils::{reflect, refract};

#[derive(Default, Copy, Clone, PartialEq)]
#[readonly::make]
pub struct Ray {
    /// origin of the ray
    pub origin: Vec3,
    /// unit vector for the direction of the ray
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction: direction.normalize() }
    }

    pub fn position_at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}

impl Ray {
    pub fn reflect(&self, normal: Vec3, point: Vec3) -> Ray {
        /*
        We take the incident vector, v, and compute the
        projection of v onto the surface normal, n. We then
        negate this value because v and n point in opposite directions.
        This gives us the "amount" of v that points in the
        direction of the normal, b. We then remove 2 * b from v
        effectively reversing the component of v that projects onto n.
        */
        Ray {
            origin: point,
            direction: reflect(self.direction, normal)
        }
    } 

    pub fn refract(&self, normal: Vec3, point: Vec3, ref_ratio: f64) -> Ray {
        Ray {
            origin: point,
            direction: refract(self.direction, normal, ref_ratio)
        }
    }
}

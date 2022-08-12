use rand::{thread_rng, Rng};

pub fn random_float() -> f64 {
    thread_rng().gen()
}

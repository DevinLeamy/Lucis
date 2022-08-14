use crate::vec3::Vec3;
use crate::utils::{random_float, u32_random_range};

#[derive(Clone)]
pub struct Perlin {
    x: Box<[u32; Perlin::POINT_COUNT as usize]>,
    y: Box<[u32; Perlin::POINT_COUNT as usize]>,
    z: Box<[u32; Perlin::POINT_COUNT as usize]>, 
    randoms: Box<[f64; Perlin::POINT_COUNT as usize]>,
}

impl Perlin {
    const POINT_COUNT: u32 = 256;
    pub fn new() -> Perlin {
        let mut randoms = Box::new([0f64; Perlin::POINT_COUNT as usize]);
        for i in 0..randoms.len() as usize {
            randoms[i] = random_float();
        }

        Perlin {
            randoms,
            x: Perlin::generate_perm(),
            y: Perlin::generate_perm(),
            z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4f64 * p.x) as i32 & 255;
        let j = (4f64 * p.y) as i32 & 255;
        let k = (4f64 * p.z) as i32 & 255;

        self.randoms[(self.x[i as usize] ^ self.y[j as usize] ^ self.z[k as usize]) as usize]
    }

    fn generate_perm() -> Box<[u32; Perlin::POINT_COUNT as usize]> {
        let mut p = Box::new([0u32; Perlin::POINT_COUNT as usize]);

        for i in 0..Perlin::POINT_COUNT as usize {
            p[i] = i as u32;
        }

        Perlin::permute(&mut p, Perlin::POINT_COUNT);

        p
    }

    /*
    Randomly permute the elements in the array p 
    */
    fn permute(p: &mut Box<[u32; Perlin::POINT_COUNT as usize]>, n: u32) {
        for i in (0..n as usize).rev() {
            let target = u32_random_range(0, i as u32);
            let temp = p[i]; 
            p[i] = p[target as usize];
            p[target as usize] = temp;
        }
    }
}

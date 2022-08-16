use crate::vec3::Vec3;
use crate::utils::{random_float, u32_random_range, t_lerp};

lazy_static! {
    static ref NOISE_GEN: Box<Perlin> = Box::new(Perlin::new());
}

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

    pub fn xor_hash(&self, i: i32, j: i32, k: i32) -> u32 {
        self.x[(i & 255) as usize] ^ self.y[(j & 255) as usize] ^ self.z[(k & 255) as usize]
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4f64 * p.x) as i32 & 255;
        let j = (4f64 * p.y) as i32 & 255;
        let k = (4f64 * p.z) as i32 & 255;

        self.randoms[(self.xor_hash(i, j, k)) as usize]
    }

    pub fn smooth_noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        // Hermitian Smoothing
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut cube = [[[0f64; 2]; 2]; 2];
        cube[0][0][0] = self.randoms[self.xor_hash(i + 0, j + 0, k + 0) as usize];
        cube[0][0][1] = self.randoms[self.xor_hash(i + 0, j + 0, k + 1) as usize];
        cube[0][1][0] = self.randoms[self.xor_hash(i + 0, j + 1, k + 0) as usize];
        cube[0][1][1] = self.randoms[self.xor_hash(i + 0, j + 1, k + 1) as usize];
        cube[1][0][0] = self.randoms[self.xor_hash(i + 1, j + 0, k + 0) as usize];
        cube[1][0][1] = self.randoms[self.xor_hash(i + 1, j + 0, k + 1) as usize];
        cube[1][1][0] = self.randoms[self.xor_hash(i + 1, j + 1, k + 0) as usize];
        cube[1][1][1] = self.randoms[self.xor_hash(i + 1, j + 1, k + 1) as usize];

        t_lerp(cube, u, v, w)
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

use crate::vec3::Vec3;
use crate::utils::{random_float, u32_random_range, t_lerp, random_range, perlin_t_lerp};

pub struct Perlin {
    x: Box<[u32; Perlin::POINT_COUNT as usize]>,
    y: Box<[u32; Perlin::POINT_COUNT as usize]>,
    z: Box<[u32; Perlin::POINT_COUNT as usize]>, 
    vectors: Box<[Vec3; Perlin::POINT_COUNT as usize]>,
}

impl Perlin {
    const POINT_COUNT: u32 = 256;
    pub fn new() -> Perlin {
        let mut vectors = Box::new([Vec3::zeros(); Perlin::POINT_COUNT as usize]);
        for i in 0..vectors.len() as usize {
            vectors[i] = Vec3::new(
                random_range(-1.0, 1.0),
                random_range(-1.0, 1.0),
                random_range(-1.0, 1.0),
            ).normalize();
        }

        Perlin {
            vectors,
            x: Perlin::generate_perm(),
            y: Perlin::generate_perm(),
            z: Perlin::generate_perm(),
        }
    }

    fn xor_hash(&self, i: i32, j: i32, k: i32) -> u32 {
        self.x[(i & 255) as usize] ^ self.y[(j & 255) as usize] ^ self.z[(k & 255) as usize]
    }

    pub fn smooth_noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();



        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut cube = [[[Vec3::zeros(); 2]; 2]; 2];
        cube[0][0][0] = self.vectors[self.xor_hash(i + 0, j + 0, k + 0) as usize];
        cube[0][0][1] = self.vectors[self.xor_hash(i + 0, j + 0, k + 1) as usize];
        cube[0][1][0] = self.vectors[self.xor_hash(i + 0, j + 1, k + 0) as usize];
        cube[0][1][1] = self.vectors[self.xor_hash(i + 0, j + 1, k + 1) as usize];
        cube[1][0][0] = self.vectors[self.xor_hash(i + 1, j + 0, k + 0) as usize];
        cube[1][0][1] = self.vectors[self.xor_hash(i + 1, j + 0, k + 1) as usize];
        cube[1][1][0] = self.vectors[self.xor_hash(i + 1, j + 1, k + 0) as usize];
        cube[1][1][1] = self.vectors[self.xor_hash(i + 1, j + 1, k + 1) as usize];

        // note: outputs may be negative! 
        perlin_t_lerp(cube, u, v, w)
    }

    fn generate_perm() -> Box<[u32; Perlin::POINT_COUNT as usize]> {
        let mut p = Box::new([0u32; Perlin::POINT_COUNT as usize]);

        for i in 0..Perlin::POINT_COUNT as usize {
            p[i] = i as u32;
        }

        Perlin::permute(&mut p, Perlin::POINT_COUNT);

        p
    }

    /// add together 'depth' layers of noise of progressively weaker weight
    pub fn turbulence(&self, p: Vec3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut weight = 1.0;
        let mut probe = p;

        for _ in 0..depth {
            acc += weight * self.smooth_noise(probe);
            weight *= 0.5;
            probe *= 2.0;
        }

        acc.abs()
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

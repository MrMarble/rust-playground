use macroquad::rand::{self, rand};

const PERLIN_YWRAPB: usize = 4;
const PERLIN_YWRAP: usize = 1 << PERLIN_YWRAPB;
const PERLIN_ZWRAPB: usize = 8;
const PERLIN_ZWRAP: usize = 1 << PERLIN_ZWRAPB;
const PERLIN_SIZE: usize = 4095;

const perlin_octaves: i32 = 4; // default to medium smooth
const perlin_amp_falloff: f32 = 0.5; // 50% reduction/octave

pub struct Perlin {
    perlin: [f32; PERLIN_SIZE + 1],
}

impl Perlin {
    pub fn new() -> Self {
        rand::srand(rand() as u64);
        let mut perlin = [0.0; PERLIN_SIZE + 1];
        for i in 0..PERLIN_SIZE {
            perlin[i] = rand::gen_range(0.0, 1.0);
        }
        Perlin { perlin }
    }

    pub fn reseed(&mut self) {
        let mut perlin = [0.0; PERLIN_SIZE + 1];
        for i in 0..PERLIN_SIZE {
            perlin[i] = rand::gen_range(0.0, 1.0);
        }

        self.perlin = perlin;
    }

    pub fn noise(&self, x: f32) -> f32 {
        self.noise_gen(x, 0.0, 0.0)
    }

    pub fn noise2d(&self, x: f32, y: f32) -> f32 {
        self.noise_gen(x, y, 0.0)
    }

    fn noise_gen(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut x = x;
        let mut y = y;
        let mut z = z;

        if x < 0.0 {
            x = -x;
        }
        if y < 0.0 {
            y = -y;
        }
        if z < 0.0 {
            z = -z;
        }

        let mut xi = x as i32;
        let mut yi = y as i32;
        let mut zi = z as i32;

        let mut xf = x - xi as f32;
        let mut yf = y - yi as f32;
        let mut zf = z - zi as f32;

        let mut rxf = 0.0;
        let mut ryf = 0.0;

        let mut r = 0.0;
        let mut ampl = 0.5;

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut n3 = 0.0;

        for _ in 0..perlin_octaves {
            let mut of =
                xi as usize + ((yi as usize) << PERLIN_YWRAPB) + ((zi as usize) << PERLIN_ZWRAPB);

            rxf = scaled_cosine(xf);
            ryf = scaled_cosine(yf);

            n1 = self.perlin[of & PERLIN_SIZE];
            n1 += rxf * (self.perlin[(of + 1) & PERLIN_SIZE] - n1);
            n2 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_SIZE];
            n2 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n2);
            n1 += ryf * (n2 - n1);

            of += PERLIN_ZWRAP;
            n2 = self.perlin[of & PERLIN_SIZE];
            n2 += rxf * (self.perlin[(of + 1) & PERLIN_SIZE] - n2);
            n3 = self.perlin[(of + PERLIN_YWRAP) & PERLIN_SIZE];
            n3 += rxf * (self.perlin[(of + PERLIN_YWRAP + 1) & PERLIN_SIZE] - n3);
            n2 += ryf * (n3 - n2);

            n1 += scaled_cosine(zf) * (n2 - n1);

            r += n1 * ampl;
            ampl *= perlin_amp_falloff;
            xi <<= 1;
            xf *= 2.0;
            yi <<= 1;
            yf *= 2.0;
            zi <<= 1;
            zf *= 2.0;

            if xf >= 1.0 {
                xi += 1;
                xf -= 1.0;
            }
            if yf >= 1.0 {
                yi += 1;
                yf -= 1.0;
            }
            if zf >= 1.0 {
                zi += 1;
                zf -= 1.0;
            }
        }
        r
    }
}

fn scaled_cosine(i: f32) -> f32 {
    0.5 * (1.0 - f32::cos(i * std::f32::consts::PI))
}

use image::{Rgb, RgbImage};
use noise::{NoiseFn, Seedable, SuperSimplex};

mod grid;
mod disc;

const OSCILLATIONS_PER_BIOME: f64 = 1.0 / 4.0;
const BIOMES_PER_PIXEL: f64 = 32.0 / 1024.0;

const COLORS: [[[Rgb<u8>; 3]; 3]; 4] = [
    [
        // deep ocean
        [Rgb([0x33, 0x66, 0x99]); 3], // cold
        [Rgb([0x33, 0x44, 0x88]); 3], // medium
        [Rgb([0x33, 0x33, 0x88]); 3], // hot
    ],
    [
        // main ocean
        [Rgb([0x77, 0xBB, 0xFF]); 3], // cold
        [Rgb([0x55, 0x88, 0xFF]); 3], // medium
        [Rgb([0x55, 0x55, 0xFF]); 3], // hot
    ],
    [
        // border land
        [
            // cold
            Rgb([0xFF, 0xFF, 0xFF]), // dry
            Rgb([0x22, 0x66, 0x22]), // medium
            Rgb([0x55, 0x99, 0xbb]), // wet
        ],
        [
            // temperate
            Rgb([0x88, 0x88, 0x88]), // dry
            Rgb([0x33, 0x88, 0x44]), // medium
            Rgb([0x00, 0xCC, 0x77]), // wet
        ],
        [
            // hot
            Rgb([0xFF, 0xFF, 0x00]), // dry
            Rgb([0xCC, 0x88, 0x33]), // medium
            Rgb([0x88, 0xFF, 0x00]), // wet
        ],
    ],
    [
        // landlocked
        [
            // cold
            Rgb([0xAA, 0xAA, 0xAA]), // dry
            Rgb([0x11, 0x44, 0x11]), // medium
            Rgb([0x44, 0x66, 0x77]), // wet
        ],
        [
            // temperate
            Rgb([0x55, 0x55, 0x55]), // dry
            Rgb([0x22, 0x55, 0x33]), // medium
            Rgb([0x11, 0x77, 0x55]), // wet
        ],
        [
            // hot
            Rgb([0x99, 0x99, 0x11]), // dry
            Rgb([0x77, 0x55, 0x33]), // medium
            Rgb([0x55, 0x99, 0x11]), // wet
        ],
    ],
];

#[derive(Clone)]
pub struct Parameters {
    world_seed: u64,
    temperature: SuperSimplex,
    wetness: SuperSimplex,
    ocean: SuperSimplex,
}

impl Parameters {
    fn color(&self, x: f64, y: f64) -> Rgb<u8> {
        let x = x * OSCILLATIONS_PER_BIOME;
        let y = y * OSCILLATIONS_PER_BIOME;
        let temp = range(self.temperature.get([x, y]));
        let wet = range(self.wetness.get([x, y]));
        let ocean = ocean_range(self.ocean.get([x / 2.0, y / 2.0]));
        COLORS[ocean][temp][wet]
    }
}

trait BiomeSampler {
    fn sample_biome_map(&self, x: f64, y: f64) -> Rgb<u8>;
}

fn main() {
    let params = Parameters {
        world_seed: 32459780,
        temperature: noise::SuperSimplex::new().set_seed(897663425),
        wetness: noise::SuperSimplex::new().set_seed(34529708),
        ocean: noise::SuperSimplex::new().set_seed(980245763),
    };

    // let sampler = NoopSampler(params.clone());
    // let sampler = grid::GridSampler::new(params.clone());
    let sampler = disc::DiscSampler::new(params.clone());

    // Output the sharp-edges biome map
    let mut img = RgbImage::new(1024, 1024);
    for (x, y, p) in img.enumerate_pixels_mut() {
        *p = sampler.sample_biome_map(x as f64 * BIOMES_PER_PIXEL, y as f64 * BIOMES_PER_PIXEL);
    }
    img.save("sharp.png").unwrap();

    // Output the turbulated (final) biome map
    let offset_x = noise::SuperSimplex::new().set_seed(567342);
    let offset_y = noise::SuperSimplex::new().set_seed(54698);
    let turbulence = 1.0 / 8.0;
    let cycles_per_pixel = 1.0 / 16.0;
    for (x, y, pix) in img.enumerate_pixels_mut() {
        let (x, y) = (x as f64, y as f64);
        let p = [x * cycles_per_pixel, y * cycles_per_pixel];
        let py = y * BIOMES_PER_PIXEL + offset_x.get(p) * turbulence;
        let px = x * BIOMES_PER_PIXEL + offset_y.get(p) * turbulence;
        *pix = sampler.sample_biome_map(px, py);
    }
    img.save("final.png").unwrap();
}

struct NoopSampler(Parameters);
impl BiomeSampler for NoopSampler {
    fn sample_biome_map(&self, x: f64, y: f64) -> Rgb<u8> {
        self.0.color(x, y)
    }
}

fn range(v: f64) -> usize {
    if v < -0.5 {
        0
    } else if v < 0.5 {
        1
    } else {
        2
    }
}

fn ocean_range(v: f64) -> usize {
    if v < -0.7 {
        0
    } else if v < 0.1 {
        1
    } else if v < 0.7 {
        2
    } else {
        3
    }
}

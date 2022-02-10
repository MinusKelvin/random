use image::Rgb;
use ordered_float::NotNan;
use rand::prelude::*;
use rand_pcg::Pcg32;

use crate::{BiomeSampler, Parameters};

const TILE_SIZE: f64 = 8.0;

pub struct DiscSampler {
    parameters: Parameters,
    points: Vec<(f64, f64)>,
}

impl DiscSampler {
    pub fn new(parameters: Parameters) -> Self {
        let mut points = vec![];
        let mut rng = Pcg32::new(parameters.world_seed, 0xa02bdbf7bb3c0a7);

        'genpoints: loop {
            'pickpoint: for _ in 0..30 {
                let x = rng.gen_range(0.0..TILE_SIZE);
                let y = rng.gen_range(0.0..TILE_SIZE);
                for &(x2, y2) in points.iter() {
                    let dx = wrapping(x - x2);
                    let dy = wrapping(y - y2);
                    let dsq = dx * dx + dy * dy;
                    if dsq < 0.8 {
                        continue 'pickpoint;
                    }
                }
                points.push((x, y));
                continue 'genpoints;
            }
            break;
        }

        DiscSampler { parameters, points }
    }
}

impl BiomeSampler for DiscSampler {
    fn sample_biome_map(&self, x: f64, y: f64) -> Rgb<u8> {
        let lx = x % TILE_SIZE;
        let ly = y % TILE_SIZE;
        let small = self.points.iter().copied().min_by_key(|(px, py)| {
            let dx = wrapping(px - lx);
            let dy = wrapping(py - ly);
            NotNan::new(dx * dx + dy * dy).unwrap()
        }).unwrap();
        let dx = wrapping(small.0 - x % TILE_SIZE);
        let dy = wrapping(small.1 - y % TILE_SIZE);
        self.parameters.color(dx + x, dy + y)
    }
}

fn wrapping(d: f64) -> f64 {
    if d > TILE_SIZE / 2.0 {
        d - TILE_SIZE
    } else if d < -TILE_SIZE / 2.0 {
        d + TILE_SIZE
    } else {
        d
    }
}

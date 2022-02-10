use image::Rgb;
use rand::prelude::*;
use rand_pcg::Pcg32;

use crate::{Parameters, BiomeSampler};

pub struct GridSampler {
    parameters: Parameters,
}

impl GridSampler {
    pub fn new(parameters: Parameters) -> Self {
        GridSampler {
            parameters
        }
    }

    fn biome_offset(&self, x: i32, y: i32) -> (f64, f64) {
        let mut rng = Pcg32::new(x as u64 | (y as u64) << 32, self.parameters.world_seed);
        rng.advance(1024);
        (rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5))
    }
}

impl BiomeSampler for GridSampler {
    fn sample_biome_map(&self, x: f64, y: f64) -> Rgb<u8> {
        let bx = x.floor();
        let by = y.floor();

        let get = |dx: i32, dy: i32| {
            let offset = self.biome_offset(bx as i32 + dx, by as i32 + dy);
            let tx = bx + dx as f64 + offset.0;
            let ty = by + dy as f64 + offset.1;
            let x = x - tx;
            let y = y - ty;
            (x * x + y * y, self.parameters.color(tx, ty))
        };
        let mut choices = [get(0, 0), get(0, 1), get(1, 1), get(1, 0)];
        choices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        choices[0].1
    }
}

use crate::engine::VertexBuffer;
use crate::planet::{
    planet_idx, Planet, REGION_HEIGHT, REGION_WIDTH,
    noise_helper::{lat_to_y, lon_to_x}
};
use parking_lot::Mutex;
mod planet_render;
use planet_render::*;

const ALTITUDE_DIVISOR: f32 = 8192.0;

lazy_static! {
    pub static ref WORLDGEN_RENDER: Mutex<WorldGenPlanetRender> =
        Mutex::new(WorldGenPlanetRender::new());
}

pub struct WorldGenPlanetRender {
    pub vertex_buffer: VertexBuffer<f32>,
    pub needs_update: bool,
}

impl WorldGenPlanetRender {
    fn new() -> Self {
        let mut wgpr = Self {
            vertex_buffer: VertexBuffer::new(&[3, 4]),
            needs_update: false,
        };
        build_blank_planet(&mut wgpr.vertex_buffer);
        wgpr
    }

    pub fn planet_with_altitude(&mut self, planet: Planet) {
        self.vertex_buffer.clear();
        all_planet_points(|l| {
            add_point(
                &mut self.vertex_buffer,
                l.0,
                l.1,
                planet.landblocks[l.2].height as f32 / ALTITUDE_DIVISOR,
                &altitude_to_color(planet.landblocks[l.2].height)
            );
        });
        self.needs_update = true;
    }

    pub fn planet_with_category(&mut self, planet: &Planet) {
        self.vertex_buffer.clear();

        all_planet_points(|l| {
            add_point(
                &mut self.vertex_buffer,
                l.0,
                l.1,
                planet.landblocks[l.2].height as f32
                    / ALTITUDE_DIVISOR,
                &landblock_to_color(
                    &planet.landblocks[l.2],
                ),
            );
        });
        self.needs_update = true;
    }

    pub fn planet_with_biome(&mut self, planet: &Planet) {
        self.vertex_buffer.clear();

        all_planet_points(|l| {
            add_point(
                &mut self.vertex_buffer,
                l.0,
                l.1,
                planet.landblocks[l.2].height as f32
                    / ALTITUDE_DIVISOR,
                &biome_to_color(l.2, &planet)
            );
        });
        self.needs_update = true;
    }

    fn hm_to_z(&self, height: u8) -> f32 {
        height as f32 / 255.0
    }

    pub fn region_heightmap(&mut self, hm: &[u8], water_level: u8, water: &[u8]) {
        self.vertex_buffer.clear();
        const SCALE: f32 = 512.0;
        const HRW: f32 = (REGION_WIDTH as f32 / 2.0) / SCALE;
        const HRH: f32 = (REGION_HEIGHT as f32 / 2.0) / SCALE;
        let min_height = hm.iter().min().unwrap();
        let max_height = hm.iter().max().unwrap();
        println!("{},{}", min_height, max_height);
        let altitude_range = max_height - min_height;

        for idx in 0..hm.len() - (REGION_WIDTH + 1) as usize {
            let height = hm[idx];
            let mag = (height - min_height) as f32 / altitude_range as f32;
            //let mag = *height as f32 / 255.0;
            let x = idx % REGION_WIDTH as usize;
            let y = idx / REGION_WIDTH as usize;

            let z00 = self.hm_to_z(height);
            let z10 = self.hm_to_z(hm[idx + 1]);
            let z01 = self.hm_to_z(hm[idx + REGION_WIDTH as usize]);
            let z11 = self.hm_to_z(hm[idx + 1 + REGION_WIDTH as usize]);

            let (r, g, b) = if height < water_level || water[idx] > height {
                (0.0, 0.0, 1.0)
            } else {
                (0.0, mag, 0.0)
            };

            let x1 = (x as f32 / SCALE) - HRW;
            let x2 = ((x + 1) as f32 / SCALE) - HRW;
            let y1 = (y as f32 / SCALE) - HRH;
            let y2 = ((y + 1) as f32 / SCALE) - HRH;

            self.vertex_buffer.add3(x1, y2, z01);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x1, y1, z00);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y1, z10);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x1, y2, z01);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y1, z10);
            self.vertex_buffer.add4(r, g, b, 1.0);
            self.vertex_buffer.add3(x2, y2, z11);
            self.vertex_buffer.add4(r, g, b, 1.0);
        }
        self.needs_update = true;
    }

    pub fn region_display_primitives(&mut self, primitives: Vec<crate::region::Primitive>) {
        self.vertex_buffer.clear();
        primitives.iter().for_each(|p| {
            match *p {
                crate::region::Primitive::Cube{x, y, z, w, h, d} => {
                    //println!("{},{},{} .. {},{},{}", x, y, z, w, h, d);
                    //self.add_cube(x, y, z, w, h, d);
                    crate::utils::add_cube_geometry(&mut self.vertex_buffer, x as f32, y as f32, z as f32, w as f32, h as f32, d as f32);
                }
            }
        });
        self.needs_update = true;
    }
}

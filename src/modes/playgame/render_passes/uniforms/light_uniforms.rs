use crate::engine::uniforms::UniformBlock;
use crate::systems::REGION;
use legion::prelude::*;
use nox_components::*;
use nox_planet::Region;
use rayon::prelude::*;
use cgmath::Vector3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LightUniforms {
    pub camera_position: [f32; 4],
    pub lights: [LightInfo; 32],
}

unsafe impl bytemuck::Pod for LightUniforms {}
unsafe impl bytemuck::Zeroable for LightUniforms {}
impl UniformBlock for LightUniforms {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightInfo {
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

impl LightInfo {
    fn new() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

unsafe impl bytemuck::Pod for LightInfo {}
unsafe impl bytemuck::Zeroable for LightInfo {}

impl LightUniforms {
    pub fn new() -> Self {
        Self {
            camera_position: [0.0, 0.0, 0.0, 0.0],
            lights: [LightInfo::new(); 32],
        }
    }

    pub fn update_partial(&mut self, sun_pos: &(Vector3<f32>, Vector3<f32>), camera_pos: &Vector3<f32>) {
        self.camera_position = vec_to_float(camera_pos);
        self.lights[0].pos = [sun_pos.0.x, sun_pos.0.y, sun_pos.0.z, 512.0];
        self.lights[0].color = [sun_pos.1.x, sun_pos.1.y, sun_pos.1.z, 1.0];
    }

    pub fn update(
        &mut self,
        ecs: &World,
        sun_pos: &(Vector3<f32>, Vector3<f32>),
        camera_pos: Vector3<f32>,
        light_bits: &mut [u32],
    ) {
        self.camera_position = vec_to_float(&camera_pos);
        self.lights[0].pos = [sun_pos.0.x, sun_pos.0.y, sun_pos.0.z, 512.0];
        self.lights[0].color = [sun_pos.1.x, sun_pos.1.y, sun_pos.1.z, 1.0];

        self.lights.iter_mut().skip(1).for_each(|l| {
            l.pos = [0.0, 0.0, 0.0, 0.0];
            l.color = [0.0, 0.0, 0.0, 0.0];
        });

        // Clear and set outdoors
        let region = REGION.read();
        light_bits.par_iter_mut().enumerate().for_each(|(idx, l)| {
            if region.flag(idx, Region::OUTSIDE) {
                *l = 1;
            } else {
                *l = 0;
            }
        });

        // Index the lights
        const LIGHT_BOOST: f32 = 5.0;
        let mut index = 1;
        let light_query = <(Read<Position>, Read<Light>, Read<FieldOfView>)>::query();
        light_query.iter(ecs).for_each(|(pos, light, fov)| {
            let pt = pos.as_point3();
            if index < 32 && pt.z <= camera_pos.y as i32 {
                self.lights[index].color = [
                    light.color.0 * LIGHT_BOOST,
                    light.color.1 * LIGHT_BOOST,
                    light.color.2 * LIGHT_BOOST,
                    0.0,
                ];
                self.lights[index].pos = [
                    pt.x as f32 + 0.5,
                    pt.z as f32 + 0.4,
                    pt.y as f32 + 0.5,
                    light.radius as f32,
                ];
                let bit = 1 << index;

                for idx in fov.visible_tiles.iter() {
                    light_bits[*idx] = light_bits[*idx] | bit;
                }
            }
            index += 1;
        });
        //println!("{:#?}", self.lights);
    }
}

#[inline]
fn vec_to_float(v: &Vector3<f32>) -> [f32; 4] {
    [v.x, v.y, v.z, 0.0]
}

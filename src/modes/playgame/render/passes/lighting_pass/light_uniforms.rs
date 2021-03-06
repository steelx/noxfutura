use bengine::gpu::util::DeviceExt;
use bengine::uv::Vec3;
use bengine::*;
use legion::*;
use nox_components::*;
use nox_planet::Region;
use rayon::prelude::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LightUniforms {
    pub screen_info: [f32; 4],
    pub camera_position: [f32; 4],
    pub lights: [LightInfo; 32],
}

unsafe impl bytemuck::Pod for LightUniforms {}
unsafe impl bytemuck::Zeroable for LightUniforms {}

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
            screen_info: [0.0, 0.0, 0.0, 0.0],
            camera_position: [0.0, 0.0, 0.0, 0.0],
            lights: [LightInfo::new(); 32],
        }
    }

    pub fn update_partial(&mut self, ecs: &World, mouse_position: &[f32]) {
        let camera_pos = <(&Position, &CameraOptions)>::query()
            .iter(ecs)
            .map(|(pos, _)| pos.as_point3())
            .nth(0)
            .unwrap();

        let mut query = <Read<Calendar>>::query();
        let mut sun_pos = (Vec3::zero(), Vec3::zero());
        for c in query.iter(ecs) {
            sun_pos = c.calculate_sun_moon();
        }

        self.screen_info[0] = mouse_position[0];
        self.screen_info[1] = mouse_position[1];

        self.camera_position = [
            camera_pos.x as f32,
            camera_pos.z as f32,
            camera_pos.y as f32,
            0.0,
        ];
        self.lights[0].pos = [sun_pos.0.x, sun_pos.0.y, sun_pos.0.z, 512.0];
        self.lights[0].color = [sun_pos.1.x, sun_pos.1.y, sun_pos.1.z, 1.0];
    }

    pub fn update(&mut self, ecs: &World, light_bits: &mut [u32], mouse_position: &[f32]) {
        self.update_partial(ecs, mouse_position);

        self.lights.iter_mut().skip(1).for_each(|l| {
            l.pos = [0.0, 0.0, 0.0, 0.0];
            l.color = [0.0, 0.0, 0.0, 0.0];
        });

        // Clear and set outdoors
        let region = crate::modes::playgame::systems::REGION.read();
        light_bits.par_iter_mut().enumerate().for_each(|(idx, l)| {
            if region.flag(idx, Region::OUTSIDE) {
                *l = 1;
            } else {
                *l = 0;
            }
        });

        // Index the lights
        const LIGHT_BOOST: f32 = 1.0;
        let mut index = 1;
        let mut light_query = <(Read<Position>, Read<Light>, Read<FieldOfView>)>::query();
        light_query.iter(ecs).for_each(|(pos, light, fov)| {
            let pt = pos.as_point3();
            if index < 32 && light.enabled {
                // pt.z <= camera_pos.y ?
                self.lights[index].color = [
                    light.color.0 * LIGHT_BOOST,
                    light.color.1 * LIGHT_BOOST,
                    light.color.2 * LIGHT_BOOST,
                    0.0,
                ];
                self.lights[index].pos = [
                    pt.x as f32 + 0.5,
                    pt.z as f32 + 0.5,
                    pt.y as f32 + 0.5,
                    light.radius as f32,
                ];
                let bit = 1 << index;

                for idx in fov.visible_tiles.iter() {
                    //println!("Setting visible tile for light {} at {}", index, idx);
                    light_bits[*idx] = light_bits[*idx] | bit;
                }
                index += 1;
            }
        });
        //println!("{:#?}", self.lights);
    }
}

pub struct LightUniformManager {
    pub uniforms: LightUniforms,
    pub uniform_buffer: gpu::Buffer,
}

impl LightUniformManager {
    pub fn new() -> Self {
        let uniforms = LightUniforms::new();

        let dl = RENDER_CONTEXT.read();
        let ctx = dl.as_ref().unwrap();

        let uniform_buffer = ctx
            .device
            .create_buffer_init(&gpu::util::BufferInitDescriptor {
                label: Some("LightUniforms"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: gpu::BufferUsage::UNIFORM | gpu::BufferUsage::COPY_DST,
            });

        Self {
            uniforms,
            uniform_buffer,
        }
    }

    pub fn send_buffer_to_gpu(&mut self) {
        let dcl = RENDER_CONTEXT.read();
        let dc = dcl.as_ref().unwrap();
        dc.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}

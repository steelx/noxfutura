mod loadstate;
mod play;
mod render;
mod systems;
mod ui;
mod uniforms;

pub use play::PlayTheGame;
pub use render::{
    Chunks, GBuffer, GrassPass, LightingPass, Models, ModelsPass, Palette, TerrainPass, VoxPass,
};
pub use uniforms::{Camera, CameraUniform};

pub struct GameStateResource {
    keycode: Option<bengine::VirtualKeyCode>,
    pub camera_changed: bool,
}

impl GameStateResource {
    pub fn new() -> Self {
        Self {
            keycode: None,
            camera_changed: false,
        }
    }

    pub fn frame_update(&mut self, keycode: Option<bengine::VirtualKeyCode>) {
        self.keycode = keycode;
    }
}

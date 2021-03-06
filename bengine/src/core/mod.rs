mod main_loop;

use imgui::Ui;

pub struct Core<'a> {
    pub imgui: &'a Ui<'a>,
    pub frame: &'a wgpu::SwapChainFrame,
    pub keycode: Option<winit::event::VirtualKeyCode>,
    pub mouse_world_pos: (usize, usize, usize),
    pub frame_time: f32,
}

pub use main_loop::run;

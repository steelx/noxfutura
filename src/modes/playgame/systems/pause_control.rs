use super::super::GameStateResource;
use bengine::VirtualKeyCode;
use legion::*;
use super::super::RunState;

#[system]
pub fn pause_control(#[resource] state: &mut GameStateResource, #[resource] run_state: &mut RunState) {
    if let Some(key) = state.keycode {
        match key {
            VirtualKeyCode::Grave => *run_state = RunState::Paused,
            VirtualKeyCode::Key1 => *run_state = RunState::SlowMo,
            VirtualKeyCode::Key2 => *run_state = RunState::Running,
            VirtualKeyCode::Key3 => *run_state = RunState::FullSpeed,
            _ => {},
        }
    }
}

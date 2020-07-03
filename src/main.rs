#[macro_use]
extern crate lazy_static;

pub mod components;
mod engine;
pub mod modes;
pub mod planet;
pub mod systems;
pub mod utils;

fn main() {
    engine::main_loop();
}

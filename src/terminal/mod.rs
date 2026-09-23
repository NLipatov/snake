#[cfg(not(target_arch = "wasm32"))]
pub mod game_loop;
#[cfg(not(target_arch = "wasm32"))]
pub mod input;
#[cfg(not(target_arch = "wasm32"))]
pub mod renderer;

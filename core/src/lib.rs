pub mod backend;
pub mod data;
pub mod renderer;
pub mod scatter;
pub mod shaders;

#[cfg(feature = "python")]
pub mod window;

pub use backend::{GPUBackend, BackendType};
pub use data::{Point2D, Color, ChartData};
pub use renderer::{Renderer, RenderOptions};
pub use scatter::ScatterRenderer;

#[cfg(feature = "python")]
pub use window::{RenderWindow, run_window};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    log::info!("Helion initialized");
}


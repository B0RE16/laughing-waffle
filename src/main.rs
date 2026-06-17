//! Cold War RTS (working title) — entry point.
//!
//! Milestone 0 scaffold: proves the fixed-timestep simulation loop + render path on
//! both native and WASM. Real systems arrive in Phase 1+ (see PLAN.md).

// Temporary: the module scaffold intentionally holds unused placeholders during
// Milestone 0. These allows are removed as systems get wired up.
#![allow(dead_code, unused_imports)]

use macroquad::prelude::*;

mod data;
mod ecs;
mod render;
mod sim;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cold War RTS (working title) — Milestone 0".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = sim::Sim::new();
    let tick_dt = 1.0 / sim::TICK_RATE as f32;
    let mut accumulator = 0.0f32;

    loop {
        // Fixed-timestep simulation, decoupled from the render framerate.
        accumulator += get_frame_time();
        while accumulator >= tick_dt {
            sim.tick();
            accumulator -= tick_dt;
        }

        // Render.
        clear_background(Color::from_rgba(18, 22, 28, 255));
        render::draw_scene(&sim);
        render::draw_debug_overlay(&sim);

        next_frame().await;
    }
}

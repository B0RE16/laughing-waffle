//! ECS components (plain data). Phase 1-2 starter set; grows each phase.

use macroquad::prelude::{Color, Vec2};

/// World-space position (in world pixels).
pub struct Position(pub Vec2);

/// How an entity draws: a sprite index (into `Sprites`), a tint, and a world size.
pub struct Renderable {
    pub sprite: usize,
    pub tint: Color,
    pub size: f32,
}

/// Which faction an entity belongs to (faction id from the data definitions).
pub struct Faction(pub String);

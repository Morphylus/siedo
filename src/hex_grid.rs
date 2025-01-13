use bevy::prelude::*;

use crate::board::BoardSettings;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_transform_with_hex_coords);
    }
}

#[derive(Component, Eq, Hash, PartialEq, Clone, Copy)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexCoord {
    pub fn to_screen_coords(self, size: f32) -> Vec3 {
        let x = size * (3_f32.sqrt() * self.q as f32 + 3_f32.sqrt() / 2.0 * self.r as f32);
        let y = size * (3.0 / 2.0 * self.r as f32);
        Vec3::new(x, y, 0.0)
    }
}

impl Default for HexCoord {
    fn default() -> Self {
        HexCoord { q: 0, r: 0, s: 0 }
    }
}

fn sync_transform_with_hex_coords(
    mut query: Query<(&HexCoord, &mut Transform)>,
    board_settings: Res<BoardSettings>,
) {
    for (hex_coord, mut transform) in &mut query {
        transform.translation = hex_coord.to_screen_coords(board_settings.tile_size);
    }
}

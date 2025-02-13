use std::ops::Add;

use bevy::{color::palettes::css::RED, prelude::*};

use crate::board::BoardSettings;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoveredTile::default())
            .add_systems(Startup, setup_hover_indicator)
            .add_systems(
                Update,
                (
                    check_hex_hover_position,
                    update_hover_indicator,
                    sync_transform_with_hex_coords,
                )
                    .chain(),
            );
    }
}

#[derive(Component, Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl HexCoord {
    pub fn to_screen_coords(self, size: f32) -> Vec3 {
        let x = size * (3_f32.sqrt() * self.q as f32 + 3_f32.sqrt() / 2.0 * self.r as f32);
        let y = -size * (3.0 / 2.0 * self.r as f32);
        Vec3::new(x, y, 0.0)
    }
}

impl Default for HexCoord {
    fn default() -> Self {
        HexCoord { q: 0, r: 0, s: 0 }
    }
}

impl Add for HexCoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        HexCoord {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

fn sync_transform_with_hex_coords(
    mut query: Query<(&HexCoord, &mut Transform)>,
    board_settings: Res<BoardSettings>,
) {
    for (hex_coord, mut transform) in &mut query {
        let screen_coords = hex_coord.to_screen_coords(board_settings.tile_size);
        transform.translation.x = screen_coords.x;
        transform.translation.y = screen_coords.y;
    }
}
fn setup_hover_indicator(mut commands: Commands) {
    commands.spawn((
        HoverIndicator,
        HexCoord { r: 0, q: 0, s: 0 },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

#[derive(Resource)]
pub struct HoveredTile {
    pub position: Option<HexCoord>,
}

impl Default for HoveredTile {
    fn default() -> Self {
        HoveredTile { position: None }
    }
}

#[derive(Component)]
struct HoverIndicator;

fn update_hover_indicator(
    hovered_tile: Res<HoveredTile>,
    board_settings: Res<BoardSettings>,
    mut gizmos: Gizmos,
    mut query: Query<(&mut HexCoord, &Transform), With<HoverIndicator>>,
) {
    let (mut entity_hex_coord, transform) = query.single_mut();
    let size = board_settings.tile_size;
    let offset = Vec2::new(transform.translation.x, transform.translation.y);

    if let Some(hovered_hex_pos) = hovered_tile.position {
        *entity_hex_coord = hovered_hex_pos;

        let mut outline_vertices: Vec<Vec2> = RegularPolygon::new(size, 6)
            .vertices(0.0)
            .into_iter()
            .collect();

        outline_vertices.push(outline_vertices.clone().into_iter().next().unwrap());
        outline_vertices = outline_vertices.iter_mut().map(|v| *v + offset).collect();

        gizmos.linestrip_2d(outline_vertices, RED);
    }
}

fn check_hex_hover_position(
    window: Query<&Window>,
    mut hovered_tile: ResMut<HoveredTile>,
    board_settings: Res<BoardSettings>,
) {
    let window = window.single();
    let window_width = window.width();
    let window_height = window.height();
    let tile_size = board_settings.tile_size;
    let board_size = board_settings.board_radius;

    if let Some(position) = window.cursor_position() {
        let centered_x = position.x - 0.5 * window_width;
        let centered_y = position.y - 0.5 * window_height;
        let frac_q = (3_f32.sqrt() / 3.0 * centered_x + -1.0 / 3.0 * centered_y) / tile_size;
        let frac_r = (2.0 / 3.0) * centered_y / tile_size;
        let frac_s = -frac_q - frac_r;

        // Rounding to nearest cube coordinate
        let mut q = frac_q.round();
        let mut r = frac_r.round();
        let mut s = frac_s.round();

        let q_diff = (q - frac_q).abs();
        let r_diff = (r - frac_r).abs();
        let s_diff = (s - frac_s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        let hex_pos = HexCoord {
            q: (q as i32),
            r: (r as i32),
            s: (s as i32),
        };

        if q.abs() as i32 <= board_size
            && r.abs() as i32 <= board_size
            && s.abs() as i32 <= board_size
        {
            hovered_tile.position = Some(hex_pos)
        } else {
            hovered_tile.position = None;
        }
    }
}

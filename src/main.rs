use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::PrimaryWindow};

mod board;
mod hex_grid;
use board::{setup_board, Board, BoardSettings};
use hex_grid::{HexCoord, HexGridPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin, HexGridPlugin))
        .insert_resource(Board::default())
        .insert_resource(BoardSettings::default())
        .insert_resource(HoveredTile::default())
        .add_systems(
            Startup,
            (setup, setup_board, spawn_settler, setup_hover_indicator),
        )
        .add_systems(Update, (check_hex_hover_position, update_hover_indicator))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
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

        if q as i32 > board_size || r as i32 > board_size || s as i32 > board_size {
            hovered_tile.position = None;
            println!("Outside of grid");
        } else {
            let hex_pos = HexCoord {
                q: (q as i32),
                r: (r as i32),
                s: (s as i32),
            };

            println!("Hex Position: {:?}", hex_pos);
            hovered_tile.position = Some(hex_pos)
        }
    }
}

fn update_hover_indicator(
    hovered_tile: Res<HoveredTile>,
    mut query: Query<&mut HexCoord, With<HoverIndicator>>,
) {
    if let Some(pointer_hex_coord) = hovered_tile.position {
        let mut entity_hex_coord = query.single_mut();

        *entity_hex_coord = pointer_hex_coord;
    }
}

fn setup_hover_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board_settings: Res<BoardSettings>,
) {
    commands.spawn((
        HoverIndicator,
        HexCoord { r: 0, q: 0, s: 0 },
        Mesh2d(meshes.add(RegularPolygon::new(board_settings.tile_size, 6))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgb(1.0, 0.0, 0.0)))),
        Transform::default(),
    ));
}

fn spawn_settler(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_coords = HexCoord { q: -1, r: -1, s: 2 };

    commands.spawn((
        GamePiece,
        PieceType::Settler,
        MovablePiece,
        MoveRange(1),
        Sprite::from_image(asset_server.load("pieces/pawn.png")),
        spawn_coords,
        Transform::default(),
    ));
}

#[derive(Component)]
struct HoverIndicator;

#[derive(Resource)]
struct HoveredTile {
    position: Option<HexCoord>,
}

impl Default for HoveredTile {
    fn default() -> Self {
        HoveredTile {
            position: Some(HexCoord { q: 0, r: -1, s: 1 }),
        }
    }
}

#[derive(Component)]
#[require(PieceType, MovablePiece, MoveRange, HexCoord)]
struct GamePiece;

#[derive(Component)]
enum PieceType {
    Settler,
}

impl Default for PieceType {
    fn default() -> Self {
        PieceType::Settler
    }
}

#[derive(Component)]
struct MovablePiece;

impl Default for MovablePiece {
    fn default() -> Self {
        MovablePiece
    }
}

#[derive(Component)]
struct MoveRange(u32);

impl Default for MoveRange {
    fn default() -> Self {
        MoveRange(1)
    }
}

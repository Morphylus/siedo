use bevy::{prelude::*, sprite::Wireframe2dPlugin, window::PrimaryWindow};

mod board;
mod hex_grid;
use board::{setup_board, Board, BoardSettings};
use hex_grid::{HexCoord, HexGridPlugin, HoveredTile};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin, HexGridPlugin))
        .insert_resource(Board::default())
        .insert_resource(BoardSettings::default())
        .add_systems(Startup, (setup, setup_board, spawn_settler))
        .add_systems(Update, piece_selection_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn piece_selection_system(
    buttons: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    game_piece: Query<(Entity, &HexCoord), With<GamePiece>>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(hex_hover_position) = hovered_tile.position {
            for (entity, hex_coord) in game_piece.iter() {
                if *hex_coord == hex_hover_position {
                    commands.entity(entity).insert(Selected);
                    println!("Selected entity at {:?}", hex_hover_position);
                }
            }
        }
    }
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
        Transform::from_xyz(0.0, 0.0, 2.0),
    ));
}

#[derive(Component)]
struct Selected;

#[derive(Component)]
struct MoveRangeIndicator;

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

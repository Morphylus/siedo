use bevy::{prelude::*, sprite::Wireframe2dPlugin};

mod board;
mod grid_system;
use board::{setup_board, Board, BoardSettings};
use grid_system::{HexCoord, HexGridPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin, HexGridPlugin))
        .insert_resource(Board::default())
        .insert_resource(BoardSettings::default())
        .add_systems(Startup, (setup, setup_board, spawn_settler))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
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

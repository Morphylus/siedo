use std::collections::HashMap;

use bevy::{ecs::system::SystemId, prelude::*, sprite::Wireframe2dPlugin};

mod board;
mod hex_grid;
use board::{setup_board, Board, BoardSettings};
use hex_grid::{HexCoord, HexGridPlugin, HoveredTile};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin, HexGridPlugin))
        .init_resource::<GameSystems>()
        .insert_resource(Board::default())
        .insert_resource(BoardSettings::default())
        .add_systems(Startup, (setup, setup_board, spawn_settler))
        .add_systems(Update, (piece_selection_system, piece_movement_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Resource)]
struct GameSystems(HashMap<String, SystemId>);

impl FromWorld for GameSystems {
    fn from_world(world: &mut World) -> Self {
        let mut game_systems = GameSystems(HashMap::new());

        game_systems.0.insert(
            "MoveRangeOverlay".into(),
            world.register_system(move_range_overlay),
        );

        game_systems.0.insert(
            "ClearMoveRangeIndicator".into(),
            world.register_system(clear_range_overlay),
        );

        game_systems
    }
}

fn move_range_overlay(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board_settings: Res<BoardSettings>,
    selected_piece: Query<(Entity, &HexCoord, &MoveRange), With<Selected>>,
    non_selected_pieces: Query<&HexCoord, (With<GamePiece>, Without<Selected>)>,
    move_range_indicators: Query<Entity, With<MoveRangeIndicator>>,
) {
    if selected_piece.is_empty() {
        for entity in move_range_indicators.iter() {
            commands.entity(entity).despawn();
        }
    } else {
        let (entity, hex_coord, move_range) = selected_piece.single();
        let range = move_range.0 as i32;
        let board_size = board_settings.board_radius;
        let tile_size = board_settings.tile_size;

        let mut in_range_hexes: Vec<HexCoord> = Vec::new();

        for q in -range..=range {
            for r in std::cmp::max(-range, -q - range)..=std::cmp::min(range, -q + range) {
                let s = -q - r;
                let overlay_location = HexCoord { q, r, s } + *hex_coord;

                if overlay_location.q.abs() <= board_size
                    && overlay_location.r.abs() <= board_size
                    && overlay_location.s.abs() <= board_size
                    && !non_selected_pieces
                        .iter()
                        .any(|piece_coord| *piece_coord == overlay_location)
                {
                    in_range_hexes.push(overlay_location);
                }
            }
        }

        for indicator_coord in in_range_hexes {
            commands.spawn((
                MoveRangeIndicator,
                indicator_coord,
                Mesh2d(meshes.add(RegularPolygon::new(tile_size, 6))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
                    0.0, 0.0, 1.0, 0.8,
                )))),
                Transform::from_xyz(0.0, 0.0, 2.0),
            ));
        }
    }
}

fn clear_range_overlay(
    move_range_indicators: Query<Entity, With<MoveRangeIndicator>>,
    mut commands: Commands,
) {
    for overlay in move_range_indicators.iter() {
        commands.entity(overlay).despawn();
    }
}

fn piece_movement_system(
    buttons: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    possible_moves: Query<&HexCoord, (With<MoveRangeIndicator>, Without<GamePiece>)>,
    mut game_piece_query: Query<(Entity, &mut HexCoord), (With<Selected>, With<GamePiece>)>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(click_position) = hovered_tile.position {
            for possible_loc in possible_moves.iter() {
                if *possible_loc == click_position {
                    let (piece, mut coords) = game_piece_query.single_mut();
                    *coords = click_position;
                }
            }
        }
    }
}

fn piece_selection_system(
    buttons: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    selected_pieces: Query<Entity, (With<GamePiece>, With<Selected>)>,
    not_selected_pieces: Query<(Entity, &HexCoord), (With<GamePiece>, Without<Selected>)>,
    systems: Res<GameSystems>,
    mut commands: Commands,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(hex_hover_position) = hovered_tile.position {
            // Unselect all game pieces
            for piece in selected_pieces.iter() {
                commands.entity(piece).remove::<Selected>();
            }

            commands.run_system(systems.0["ClearMoveRangeIndicator"]);

            // Select relevant piece
            for (piece, piece_coord) in not_selected_pieces.iter() {
                if *piece_coord == hex_hover_position {
                    commands.entity(piece).insert(Selected);
                }
            }
            commands.run_system(systems.0["MoveRangeOverlay"]);
        }
    }
}

fn spawn_settler(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spawn_coords = HexCoord { q: -1, r: -1, s: 2 };

    commands.spawn((
        GamePiece,
        PieceType::Settler,
        MovablePiece,
        MoveRange(3),
        Sprite::from_image(asset_server.load("pieces/pawn.png")),
        spawn_coords,
        Transform::from_xyz(0.0, 0.0, 3.0),
    ));

    commands.spawn((
        GamePiece,
        PieceType::Settler,
        MovablePiece,
        MoveRange(2),
        Sprite::from_image(asset_server.load("pieces/sword.png")),
        HexCoord { q: 0, r: 0, s: 0 },
        Transform::from_xyz(0.0, 0.0, 3.0),
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

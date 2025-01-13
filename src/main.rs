use bevy::{prelude::*, sprite::Wireframe2dPlugin};

mod board;
use board::{setup_board, Board, BoardSettings};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .insert_resource(Board::default())
        .insert_resource(BoardSettings::default())
        .add_systems(Startup, (setup, setup_board))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

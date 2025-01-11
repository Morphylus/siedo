use std::collections::HashMap;

use bevy::{prelude::*, sprite::Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .insert_resource(Grid::default())
        .add_systems(Startup, (setup, setup_grid))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 3;
    let tile_size = 50.0;

    for q in -radius..=radius {
        let r1 = std::cmp::max(-radius, -q - radius);
        let r2 = std::cmp::min(radius, -q + radius);
        for r in r1..=r2 {
            let coord = HexCoord { q, r, s: -q - r };
            let pixel_coord = coord.center_pixel_coord(tile_size);

            let tile = commands
                .spawn((
                    Tile,
                    coord.clone(),
                    Transform::from_xyz(pixel_coord.x, pixel_coord.y, 0.0),
                    GlobalTransform::default(),
                    Mesh2d(meshes.add(RegularPolygon::new(tile_size, 6))),
                    MeshMaterial2d(
                        materials.add(ColorMaterial::from(Color::linear_rgb(1.0, 0.0, 0.0))),
                    ),
                ))
                .id();
            grid.add_tile(coord, tile);
        }
    }
}

#[derive(Resource)]
struct Grid {
    tiles: HashMap<HexCoord, Entity>,
}

impl Grid {
    fn add_tile(&mut self, coord: HexCoord, entity: Entity) {
        self.tiles.insert(coord, entity);
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            tiles: HashMap::new(),
        }
    }
}

#[derive(Component, Eq, Hash, PartialEq, Clone, Copy)]
struct HexCoord {
    q: i32,
    r: i32,
    s: i32,
}

#[derive(Component)]
struct Tile;

struct Coord2D {
    x: f32,
    y: f32,
}

impl HexCoord {
    fn center_pixel_coord(self, size: f32) -> Coord2D {
        let x = size * (3_f32.sqrt() * self.q as f32 + 3_f32.sqrt() / 2.0 * self.r as f32);
        let y = size * (3.0 / 2.0 * self.r as f32);
        return Coord2D { x, y };
    }
}

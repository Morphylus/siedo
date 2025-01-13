use bevy::{prelude::*, utils::HashMap};
use rand::Rng;

#[derive(Resource)]
pub struct BoardSettings {
    tile_size: f32,
    board_radius: i32,
    gold_pr: f32,
    wheat_pr: f32,
    stone_pr: f32,
    wood_pr: f32,
}

impl Default for BoardSettings {
    fn default() -> Self {
        BoardSettings {
            tile_size: 20.0,
            board_radius: 10,
            gold_pr: 0.05,
            wheat_pr: 0.4,
            stone_pr: 0.2,
            wood_pr: 0.35,
        }
    }
}

#[derive(Resource)]
pub struct Board {
    tiles: HashMap<HexCoord, Entity>,
}

impl Board {
    fn add_tile(&mut self, coord: HexCoord, entity: Entity) {
        self.tiles.insert(coord, entity);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            tiles: HashMap::new(),
        }
    }
}

pub fn setup_board(
    mut commands: Commands,
    mut grid: ResMut<Board>,
    level_settings: Res<BoardSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = level_settings.board_radius;
    let tile_size = level_settings.tile_size;

    for q in -radius..=radius {
        let r1 = std::cmp::max(-radius, -q - radius);
        let r2 = std::cmp::min(radius, -q + radius);
        for r in r1..=r2 {
            let coord = HexCoord { q, r, s: -q - r };
            let pixel_coord = coord.center_pixel_coord(tile_size);
            let mut rng = rand::thread_rng();
            let random_value: f32 = rng.gen();

            let resource_type = match random_value {
                x if x < level_settings.gold_pr => Resource::Gold,
                x if x < level_settings.gold_pr + level_settings.wheat_pr => Resource::Wheat,
                x if x < level_settings.gold_pr
                    + level_settings.wheat_pr
                    + level_settings.stone_pr =>
                {
                    Resource::Stone
                }
                _ => Resource::Wood,
            };

            let tile = commands
                .spawn((
                    Tile,
                    resource_type,
                    coord.clone(),
                    Transform::from_xyz(pixel_coord.x, pixel_coord.y, 0.0),
                    GlobalTransform::default(),
                    Mesh2d(meshes.add(RegularPolygon::new(tile_size, 6))),
                    MeshMaterial2d(materials.add(resource_type.get_color())),
                ))
                .id();
            grid.add_tile(coord, tile);
        }
    }
}

#[derive(Component)]
#[require(Resource)]
struct Tile;

#[derive(Component, Eq, Hash, PartialEq, Clone, Copy)]
struct HexCoord {
    q: i32,
    r: i32,
    s: i32,
}

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

#[derive(Component, Copy, Clone)]
enum Resource {
    Gold,
    Wheat,
    Stone,
    Wood,
}

impl Resource {
    fn get_color(self) -> ColorMaterial {
        match self {
            Resource::Gold => ColorMaterial::from_color(Color::linear_rgb(1.0, 0.8, 0.0)),
            Resource::Stone => ColorMaterial::from_color(Color::linear_rgb(0.4, 0.4, 0.6)),
            Resource::Wheat => ColorMaterial::from_color(Color::linear_rgb(1.0, 1.0, 0.4)),
            Resource::Wood => ColorMaterial::from_color(Color::linear_rgb(0.6, 0.4, 0.2)),
        }
    }
}

impl Default for Resource {
    fn default() -> Self {
        Resource::Wheat
    }
}

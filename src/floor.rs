use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::unit::Unit;

pub enum TileKind {
    Floor,
    Wall,
    Lava,
}

impl TileKind {}

pub struct Tile {
    pub x: u8,
    pub y: u8,
    pub tile_kind: TileKind,
}

impl Tile {
    fn new(x: u8, y: u8, tile_kind: TileKind) -> Self {
        Tile { x, y, tile_kind }
    }

    fn get_color(&self) -> Color {
        let white = Color::rgb(1., 0.9, 0.9);
        let black = Color::rgb(0., 0.1, 0.1);
        let red = Color::rgb(1., 0.0, 0.0);

        match self.tile_kind {
            TileKind::Floor => white.clone(),
            TileKind::Wall => black.clone(),
            TileKind::Lava => red.clone(),
        }
    }
}

#[derive(Default)]
pub struct TargetTile {
    entity: Option<Entity>,
}

fn floor_highlights(
    pick_state: Res<PickState>,
    targeted_tile: Res<TargetTile>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Tile, &Handle<StandardMaterial>)>,
) {
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, tile, material_handle) in query.iter() {
        // TODO: Why does this have to be an unwrap?
        let material = materials.get_mut(material_handle).unwrap();

        material.albedo = if Some(entity) == top_entity {
            Color::rgb(0.5, 0.5, 1.)
        } else if Some(entity) == targeted_tile.entity {
            Color::rgb(0.0, 0.0, 1.)
        } else {
            tile.get_color()
        };
    }
}

fn target_tile(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut targeted_tile: ResMut<TargetTile>,
    tiles_query: Query<&Tile>,
    mut units_query: Query<(Entity, &mut Unit)>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        targeted_tile.entity = Some(*entity);
        let tile = tiles_query.get(*entity).unwrap();

        for (_entity, mut unit) in units_query.iter_mut() {
            unit.target_x = tile.x as f32;
            unit.target_y = tile.y as f32;
        }

        targeted_tile.entity = None;
    } else {
        targeted_tile.entity = None
    };
}

pub fn create_floor(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    let floor_design = vec![
        vec![0, 0, 0, 0, 0, 0, 2, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
    ];

    for (i, row) in floor_design.iter().rev().enumerate() {
        for (j, column) in row.iter().enumerate() {
            let tile_kind = match column {
                0 => TileKind::Floor,
                1 => TileKind::Wall,
                2 => TileKind::Lava,
                _ => panic!("I don't know what this tile is"),
            };

            // TODO: figure out why these are flipped?
            let tile = Tile::new(i as u8, j as u8, tile_kind);
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(tile.get_color().into()),
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(tile);
        }
    }
}

pub struct FloorPlugin;
impl Plugin for FloorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TargetTile>()
            .add_startup_system(create_floor.system())
            .add_system(floor_highlights.system())
            .add_system(target_tile.system());
    }
}

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct PlayerDestination {
    pub x: f32,
    pub z: f32,
}

fn create_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");

    // Add some materials
    // let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0.3, 0.3, 0.3).into());

    spawn_player(
        commands,
        black_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        (0., 0.),
    );
}

fn spawn_player(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    (x, z): (f32, f32),
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x, 0., z)),
            ..Default::default()
        })
        .with(Player { x, z })
        .with(PlayerDestination { x, z })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn move_player(time: Res<Time>, mut query: Query<(&Player, &PlayerDestination, &mut Transform)>) {
    for (piece, destination, mut transform) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(destination.x, 0., destination.z) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_player.system())
            .add_system(move_player.system());
    }
}

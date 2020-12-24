use bevy::prelude::*;

pub struct Unit {
    pub target_x: f32,
    pub target_y: f32,
}

fn move_units(time: Res<Time>, mut query: Query<(&mut Transform, &Unit)>) {
    for (mut transform, unit) in query.iter_mut() {
        let speed = 1.;
        let direction = Vec3::new(unit.target_x, 0., unit.target_y) - transform.translation;

        if direction.length_squared() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds() * speed;
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(Unit {
            target_x: position.x,
            target_y: position.y,
        })
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

pub fn spawn_enemy(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands
        // Spawn parent entity
        .spawn(PbrBundle {
            transform: Transform::from_translation(position),
            ..Default::default()
        })
        .with(Unit {
            target_x: position.x,
            target_y: position.y,
        })
        // Add children to the parent
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

pub fn create_units(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let _knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let _knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let _rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let _bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let _queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    spawn_player(
        commands,
        white_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(0., 0., 4.),
    );

    spawn_enemy(
        commands,
        black_material.clone(),
        pawn_handle.clone(),
        Vec3::new(7., 0., 3.),
    );
}

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_units.system())
            .add_system(move_units.system());
    }
}

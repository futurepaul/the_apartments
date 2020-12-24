use bevy::prelude::*;
use bevy_mod_picking::*;

mod player;
use player::*;
mod world;
use world::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_startup_system(setup.system())
        .add_system(event_example.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        //plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with(PickableMesh::default())
        .with(InteractableMesh::default())
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn event_example(query: Query<(&InteractableMesh, Entity)>) {
    for (interactable, entity) in &mut query.iter() {
        // let is_hovered = interactable.hover(&Group::default()).unwrap();
        // let hover_event = interactable.hover_event(&Group::default()).unwrap();
        let mouse_down_event = interactable
            .mouse_down_event(&Group::default(), MouseButton::Left)
            .unwrap();
        // Only print updates if at least one event has occured.
        if mouse_down_event.is_none() {
            continue;
        }
        println!("ENTITY: {:?}, CLICK_EVENT: {:?}", entity, mouse_down_event);
    }
}

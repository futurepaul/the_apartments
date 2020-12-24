use bevy::prelude::*;
use bevy_mod_picking::*;

mod unit;
use unit::{create_units, UnitPlugin};

mod floor;
use floor::{create_floor, FloorPlugin};

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7., 20., 4.),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "The Apartments".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        //        .add_plugin(DebugPickingPlugin)
        .add_plugin(FloorPlugin)
        .add_plugin(UnitPlugin)
        .add_startup_system(setup.system())
        .run();
}

use crate::player::*;
use bevy::{app::AppExit, prelude::*};
use bevy_mod_picking::*;

struct Destination {
    x: f32,
    z: f32,
}

impl Default for Destination {
    fn default() -> Self {
        Destination { x: 2., z: 4. }
    }
}

fn move_player(
    commands: &mut Commands,
    destination: ChangedRes<Destination>,
    // mut selected_square_mut: ResMut<SelectedSquare>,
    // mut selected_piece: ResMut<SelectedPiece>,
    // mut turn: ResMut<PlayerTurn>,
    // squares_query: Query<&Square>,
    mut player_query: Query<(Entity, &mut PlayerDestination)>,
) {
    for (entity, mut playerdestination) in player_query.iter_mut() {
        playerdestination.x = destination.x;
        playerdestination.z = destination.z;
        // player.x = 2.;
        // player.z = 2.;
    }
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Destination>()
            .add_system(move_player.system());
    }
}

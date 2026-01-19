mod pieces;
mod bevy_setup;
mod board_utils;
mod bevy_systems;

use bevy_setup::setup;
use bevy_systems::{handle_keyboard_input, handle_rotation};
use bevy::prelude::*;

use crate::{bevy_systems::handle_next_piece, pieces::PieceMap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PieceMap::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_next_piece,
            handle_rotation,
            handle_keyboard_input,
        ))
        .run();
}

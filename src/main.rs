mod pieces;
mod bevy_impl;
mod board_utils;

use ndarray::Array2;
use bevy_impl::setup;

use bevy::{
    input::common_conditions::input_just_pressed,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
};
use bevy::{input::common_conditions::input_toggle_active, prelude::*};

fn arr2str(arr: &Array2<String>) -> String {
    let (rows, cols) = arr.dim();
    let mut result = String::new();
    
    for i in 0..rows {
        for j in 0..cols {
            result.push_str(&arr[[i, j]]);
        }
        if i < rows - 1 {
            result.push('\n');
        }
    }
    result
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        Wireframe2dPlugin::default(),
    ))
    .add_systems(Startup, setup);
    app.add_systems(
        Update,
        toggle_wireframe.run_if(input_just_pressed(KeyCode::Space)),
    );
    app.add_systems(
        Update,
        rotate.run_if(input_toggle_active(false, KeyCode::KeyR)),
    );
    app.run();
}

fn toggle_wireframe(mut wireframe_config: ResMut<Wireframe2dConfig>) {
    wireframe_config.global = !wireframe_config.global;
}

fn rotate(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() / 2.0);
    }
}
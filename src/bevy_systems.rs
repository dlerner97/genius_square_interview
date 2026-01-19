use bevy::prelude::*;
use crate::pieces::{BasePiece, PieceMap, add_piece_to_board};

pub fn handle_next_piece(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut piece_map: ResMut<PieceMap>,
    mut query: Query<(Entity, &mut BasePiece)>,
) {
    // Handle Q presses
    if keyboard.just_pressed(KeyCode::KeyQ) {
        println!("Pressed Q!");
        
        for (entity, mut base_piece) in query.iter_mut() {
            if !base_piece.is_active() {
                continue;
            }
            commands.entity(entity).despawn();
            base_piece.set_active(false);
            println!("Despawned entity: {:?}", entity);
        }
        
        if let Some(next_piece) = piece_map.next_piece() {
            let mut active_piece = next_piece.clone();
            active_piece.set_active(true);
            add_piece_to_board(&mut commands, &mut meshes, &mut materials, &mut active_piece, (-450., 0.));
        }
    }
}

pub fn handle_rotation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut piece_map: ResMut<PieceMap>,
    mut query: Query<(Entity, &mut BasePiece, &Transform)>,
) {
    // Handle R key presses
    if keyboard.just_pressed(KeyCode::KeyR) {
        println!("Pressed R!");
        
        let mut position = Vec3::new(-450., 0., 0.);
        for (_entity, base_piece, transform) in query.iter() {
            if base_piece.is_active() {
                position = transform.translation;
                break;
            }
        }
        
        // TODO: Need better protection from multiple active
        for (entity, mut base_piece, _) in query.iter_mut() {
            if !base_piece.is_active() {
                continue;
            }
            commands.entity(entity).despawn();
            base_piece.set_active(false);
        }
        

        piece_map.rotate_current();
        if let Some(curr_piece) = piece_map.current_piece() {
            let mut active_piece: BasePiece = curr_piece.clone();
            active_piece.set_active(true);
            add_piece_to_board(&mut commands, &mut meshes, &mut materials, &mut active_piece, position.truncate().into());
        }
    }
}

pub fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut BasePiece, &mut Transform)>,
) {
    // Handle WASD key presses
    const BLOCK_SIZE: f32 = 100.;
    
    for (mut base_piece, mut transform) in query.iter_mut() {
        if !base_piece.is_active() {
            continue;
        }
        
        if keyboard.just_pressed(KeyCode::KeyW) {
            base_piece.shift_up();
            transform.translation.y += BLOCK_SIZE;
        }
        if keyboard.just_pressed(KeyCode::KeyS) {
            base_piece.shift_down();
            transform.translation.y -= BLOCK_SIZE;
        }
        if keyboard.just_pressed(KeyCode::KeyA) {
            base_piece.shift_left();
            transform.translation.x -= BLOCK_SIZE;
        }
        if keyboard.just_pressed(KeyCode::KeyD) {
            base_piece.shift_right();
            transform.translation.x += BLOCK_SIZE;
        }
    }
}
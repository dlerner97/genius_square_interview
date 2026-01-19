use bevy::prelude::*;
use crate::{board_utils::{BOARD_SIZE_PX, DEFAULT_BLOCKER_POSITIONS, GRID_SIZE, GRID_START_PX, THICKNESS, assert_valid_location, px_from_grid_loc}, pieces::PieceMap};
use crate::pieces::gen_pieces;

fn spawn_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the full board
    let outer_board_shape = meshes.add(Rectangle::new(BOARD_SIZE_PX.0, BOARD_SIZE_PX.1).to_ring(THICKNESS));
    let color = Color::hsl(0., 1., 1.);
    commands.spawn((
        Mesh2d(outer_board_shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(200., 0., 0.),
    ));

    let mut inner_board_shapes = vec![];
    for _i in 0..GRID_SIZE.0 {
        for _j in 0..GRID_SIZE.1 {
            inner_board_shapes.push(meshes.add(Rectangle::new(100., 100.).to_ring(THICKNESS / 2.)));
        }
    }
    
    let mut starting_col = GRID_START_PX.1 - 100.;
    let starting_row = GRID_START_PX.0 - 100.;
    for (i, shape) in inner_board_shapes.into_iter().enumerate() {
        let remainder = i % GRID_SIZE.0;
        if remainder == 0 {
            starting_col += 100.;
        }

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                starting_col,
                starting_row + (100. * (remainder as f32)),
                0.0,
            ),
        ));
    }

}

fn spawn_blockers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    positions: Option<&[(i32, i32)]>,
) {
    // Spawn blockers at designated positions
    let positions = positions.unwrap_or(DEFAULT_BLOCKER_POSITIONS);
    if positions.len() != 7 {
        panic!("Must have 7 blockers")
    }
    assert_valid_location(positions);
    let color = Color::linear_rgb(129./255., 84./255., 56./255.);

    for (row, col) in positions {
        let blocker = meshes.add(Circle::new(75.0 / 2.));
        let (row_px, col_px) = px_from_grid_loc((*row, *col));
        commands.spawn((
            Mesh2d(blocker),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                col_px as f32,
                row_px as f32,
                0.,
            ),
        ));

    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut piece_map: ResMut<PieceMap>,
) {
    // Setup routine upon bevy start
    // https://bevy.org/examples/2d-rendering/2d-shapes/
    commands.spawn(Camera2d);
    spawn_board(&mut commands, &mut meshes, &mut materials);
    spawn_blockers(&mut commands, &mut meshes, &mut materials, None);
    *piece_map = gen_pieces();

    let mut text = "Q - Next Piece".to_string();
    text.push_str("\nWASD - Move Piece");
    text.push_str("\nR - Rotate Piece");
    text.push_str("\nEnter - NOT IMPLEMENTED");

    commands.spawn((
        Text::new(text),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

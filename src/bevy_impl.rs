

// https://bevy.org/examples/2d-rendering/2d-shapes/

use bevy::{
    input::common_conditions::input_just_pressed,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
};
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use crate::board_utils::{GRID_SIZE, X_EXTENT, Y_EXTENT, THICKNESS};

const DEFAULT_BLOCKER_POSITIONS: &[(i32, i32)] = &[
    (0, 0),
    (0, 1),
    (2, 2),
    (4, 4),
    (1, 3),
    (0, 5),
    (4, 2),
];

const BOARD_SIZE_PX: (f32, f32) = (700., 700.);
const GRID_START_PX: (f32, f32) = ((-BOARD_SIZE_PX.0 + 400.) / 2., (-BOARD_SIZE_PX.1 + 200.) / 2.);

fn assert_valid_location(positions: &[(i32, i32)]) {
    for (row, col) in positions {
        if *row < 0 || *row >= (GRID_SIZE.0 as i32) || *col < 0 || *col >= (GRID_SIZE.1 as i32) {
            panic!("Illegal index");
        }
    }
}

fn px_from_grid_loc(position: (i32, i32)) -> (i32, i32) {
    assert_valid_location(&[position]);
    let (row, col) = position;
    let row_px = (GRID_START_PX.0 + 100. * (row as f32)) as i32;
    let col_px = (GRID_START_PX.1 + 100. * (col as f32)) as i32;
    (row_px, col_px)
}

fn spawn_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let outer_board_shape = meshes.add(Rectangle::new(BOARD_SIZE_PX.0, BOARD_SIZE_PX.1).to_ring(THICKNESS));
    let color = Color::hsl(0., 1., 1.);
    commands.spawn((
        Mesh2d(outer_board_shape),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., 0., 0.),
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
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
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

fn spawn_pieces(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_board(&mut commands, &mut meshes, &mut materials);
    spawn_blockers(&mut commands, &mut meshes, &mut materials, None);


    // let rings = [
    //     meshes.add(Circle::new(50.0).to_ring(THICKNESS)),
    //     // this visually produces an arc segment but this is not technically accurate
    //     meshes.add(Ring::new(
    //         CircularSector::new(50.0, 1.0),
    //         CircularSector::new(45.0, 1.0),
    //     )),
    //     meshes.add(CircularSegment::new(50.0, 1.25).to_ring(THICKNESS)),
    //     meshes.add({
    //         // This is an approximation; Ellipse does not implement Inset as concentric ellipses do not have parallel curves
    //         let outer = Ellipse::new(25.0, 50.0);
    //         let mut inner = outer;
    //         inner.half_size -= Vec2::splat(THICKNESS);
    //         Ring::new(outer, inner)
    //     }),
    //     // this is equivalent to the Annulus::new(25.0, 50.0) above
    //     meshes.add(Ring::new(Circle::new(50.0), Circle::new(25.0))),
    //     meshes.add(Capsule2d::new(25.0, 50.0).to_ring(THICKNESS)),
    //     meshes.add(Rhombus::new(75.0, 100.0).to_ring(THICKNESS)),
    //     meshes.add(Rectangle::new(50.0, 100.0).to_ring(THICKNESS)),
    //     meshes.add(RegularPolygon::new(50.0, 6).to_ring(THICKNESS)),
    //     meshes.add(
    //         Triangle2d::new(
    //             Vec2::Y * 50.0,
    //             Vec2::new(-50.0, -50.0),
    //             Vec2::new(50.0, -50.0),
    //         )
    //         .to_ring(THICKNESS),
    //     ),
    // ];
    // Allow for 2 empty spaces
    // let num_rings = rings.len() + 2;

    // for (i, shape) in rings.into_iter().enumerate() {
    //     // Distribute colors evenly across the rainbow.
    //     let color = Color::hsl(360. * i as f32 / num_rings as f32, 0.95, 0.7);

    //     commands.spawn((
    //         Mesh2d(shape),
    //         MeshMaterial2d(materials.add(color)),
    //         Transform::from_xyz(
    //             // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
    //             -X_EXTENT / 2. + i as f32 / (num_rings - 1) as f32 * X_EXTENT,
    //             -Y_EXTENT / 2.,
    //             0.0,
    //         ),
    //     ));
    // }

    let mut text = "Press 'R' to pause/resume rotation".to_string();
    text.push_str("\nPress 'Space' to toggle wireframes");

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
use ndarray::Array2;
use bevy::prelude::*;

pub const GRID_SIZE: (usize, usize) = (6, 6);
pub const THICKNESS: f32 = 5.0;
pub const BOARD_SIZE_PX: (f32, f32) = (700., 700.);
pub const GRID_START_PX: (f32, f32) = ((-BOARD_SIZE_PX.0 + 400.) / 2., (-BOARD_SIZE_PX.1 + 600.) / 2.);
pub const BLOCK_SIZE: f32 = 75.;


pub const DEFAULT_BLOCKER_POSITIONS: &[(i32, i32)] = &[
    (0, 0),
    (0, 1),
    (2, 2),
    (4, 4),
    (1, 3),
    (0, 5),
    (4, 2),
];


pub fn assert_valid_location(positions: &[(i32, i32)]) {
    for (row, col) in positions {
        if *row < 0 || *row >= (GRID_SIZE.0 as i32) || *col < 0 || *col >= (GRID_SIZE.1 as i32) {
            panic!("Illegal index");
        }
    }
}

pub fn px_from_grid_loc(position: (i32, i32)) -> (i32, i32) {
    assert_valid_location(&[position]);
    let (row, col) = position;
    let row_px = (GRID_START_PX.0 + 100. * (row as f32)) as i32;
    let col_px = (GRID_START_PX.1 + 100. * (col as f32)) as i32;
    (row_px, col_px)
}

pub fn arr2str(arr: &Array2<String>) -> String {
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
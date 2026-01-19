use crate::board_utils::{BLOCK_SIZE, GRID_SIZE};
use ndarray::Array2;
use std::collections::HashMap;
use bevy::{
    asset::Assets,
    camera::visibility::Visibility,
    color::Color,
    ecs::{
        entity::Entity,
        resource::Resource,
        system::{Commands, ResMut}
    }, math::primitives::Rectangle,
    mesh::{Mesh, Mesh2d},
    prelude::Component,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform
};

// We derive this as a bevy
#[derive(Component, Clone)]
pub struct BasePiece {
    // This represents the BasePiece concept. With this container, we can fully encapsulate the state of a given piece
    // including rotation, position, and whether the piece is 'active'
    piece: Array2<String>,
    board_pos_row: usize,
    board_pos_col: usize,
    active: bool, // The active piece is the one a player is actively manipulating. There should only ever be one with a 'true' value
}

impl BasePiece {
    pub fn new(piece: Array2<String>) -> Self {
        BasePiece {
            piece,
            board_pos_row: 0,
            board_pos_col: 0,
            active: false,
        }
    }
    
    // TODO: These shift functions should snap to the various grid squares.
    pub fn shift_up(&mut self) {
        if self.board_pos_row > 0 {
            self.board_pos_row -= 1;
        }
    }
    
    pub fn shift_down(&mut self) {
        if self.board_pos_row < GRID_SIZE.0 {
            self.board_pos_row += 1;
        }
    }
    
    pub fn shift_left(&mut self) {
        if self.board_pos_col > 0 {
            self.board_pos_col -= 1;
        }
    }
    
    pub fn shift_right(&mut self) {
        if self.board_pos_col < GRID_SIZE.1 {
            self.board_pos_col += 1;
        }
}
    
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    
    pub fn is_active(&self) -> bool {
        self.active
    }

    // TODO: Does not account for symmetry over piece xy-axes
    pub fn rotate(&mut self) {
        // https://stackoverflow.com/questions/42519/how-do-you-rotate-a-two-dimensional-array
        let transposed = self.piece.t().to_owned(); // Python: deepcopy
        let (rows, cols) = transposed.dim();
        let mut result = transposed;
        for i in 0..rows {
            for j in 0..cols / 2 {
                let temp = result[[i, j]].clone(); // More deepcopies... Eeek
                result[[i, j]] = result[[i, cols - 1 - j]].clone();
                result[[i, cols - 1 - j]] = temp;
            }
        }
        self.piece = result;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PieceType {
    LPiece, SqPiece, Str4Piece, Str3Piece, Str2Piece, Str1Piece, ZPiece, TPiece, CornerPiece
}

#[derive(Resource)]
pub struct PieceMap {
    // This collection stores all game pieces
    pub map: HashMap<PieceType, BasePiece>,
    pub keys: Vec<PieceType>,
    pub current_type: Option<PieceType>,
}

impl Default for PieceMap {
    fn default() -> Self {
        PieceMap {
            map: HashMap::new(),
            keys: Vec::new(),
            current_type: None,
        }
    }
}

impl PieceMap {
    pub fn new(map: HashMap<PieceType, BasePiece>) -> Self {
        let keys: Vec<PieceType> = map.keys().cloned().collect();
        let current_type = keys.first().cloned();
        
        PieceMap {
            map,
            keys,
            current_type,
        }
    }
    
    pub fn next_piece(&mut self) -> Option<&BasePiece> {
        if self.keys.is_empty() {
            return None;
        }
        
        let mut current_index = 0;
        if let Some(curr_type) = self.current_type {
            for (index, key) in self.keys.iter().enumerate() {
                if *key == curr_type {
                    current_index = index;
                    break;
                }
            }
        }
        
        let next_index = (current_index + 1) % self.keys.len();
        let key = self.keys[next_index];
        self.current_type = Some(key);
        self.map.get(&key)
    }
    
    pub fn current_piece(&self) -> Option<&BasePiece> {
        self.current_type
            .and_then(|key| self.map.get(&key))
    }

    pub fn rotate_current(&mut self) {
        if let Some(current_type) = self.current_type {
            if let Some(piece) = self.map.get_mut(&current_type) {
                piece.rotate();
            }
        }
    }
}

pub fn gen_pieces() -> PieceMap {
    // Generate all BasePiece components
    let piece_map = HashMap::from(
        [ 
            (
                PieceType::LPiece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (3, 2), 
                        vec!["1".to_string(), "1".to_string(), "0".to_string(), 
                            "1".to_string(), "0".to_string(), "1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::SqPiece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (2, 2), 
                        vec!["1".to_string(), "1".to_string(), "1".to_string(), "1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::Str4Piece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (1, 4), 
                        vec!["1".to_string(), "1".to_string(), "1".to_string(), "1".to_string()]
                    ).unwrap(),
                ),
            ),
            (
                PieceType::Str3Piece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (1, 3), 
                        vec!["1".to_string(), "1".to_string(), "1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::Str2Piece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (1, 2), 
                        vec!["1".to_string(), "1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::Str1Piece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (1, 1), 
                        vec!["1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::ZPiece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (3, 2), 
                        vec!["1".to_string(), "0".to_string(), "1".to_string(), 
                            "1".to_string(), "0".to_string(), "1".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::TPiece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (3, 2), 
                        vec!["1".to_string(), "0".to_string(), "1".to_string(), 
                            "1".to_string(), "1".to_string(), "0".to_string()]
                    ).unwrap(),
                )
            ),
            (
                PieceType::CornerPiece,
                BasePiece::new(
                    Array2::from_shape_vec(
                        (2, 2), 
                        vec!["1".to_string(), "1".to_string(), "1".to_string(), "0".to_string()]
                    ).unwrap(),
                )
            ),
        ]);
    PieceMap::new(piece_map)
}

pub fn build_tetris_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    base_piece: &BasePiece,
    color: Option<Color>,
) -> Entity {
    // Form our given shape using bevy's parent-child bundles and the corresponding piece vector
    let (rows, cols) = base_piece.piece.dim();
    let block_mesh = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
    let material = materials.add(color.unwrap_or(Color::linear_rgb(0.5, 0.5, 0.5)));
    
    let parent = commands.spawn((
        base_piece.clone(),
        Transform::from_xyz(0., 0., 0.),
        Visibility::Visible,
    )).id();
    
    for i in 0..rows {
        for j in 0..cols {
            if base_piece.piece[[i, j]] == "1" {
                let x = j as f32 * BLOCK_SIZE;
                let y = -(i as f32 * BLOCK_SIZE);
                
                let block = commands.spawn((
                    Mesh2d(block_mesh.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(x, y, 0.0),
                )).id();
                
                commands.entity(parent).add_child(block);
            }
        }
    }
    parent
}

pub fn add_piece_to_board(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    piece: &mut BasePiece,
    loc: (f32, f32),
) -> Entity {
    // Display piece at provided pixel location
    let entity = build_tetris_shape(
        commands,
        meshes,
        materials,
        piece,
        Some(Color::srgb(1.0, 0.5, 0.0)),
    );

    commands.entity(entity).insert(
        Transform::from_xyz(loc.0, loc.1, 0.0)
    );
    entity
}

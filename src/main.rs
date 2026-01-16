mod pieces;
use ndarray::Array2;
use bevy::prelude::*;

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
    // App::new().run();

}


use ndarray::Array2;

const GRID_SIZE: (usize, usize) = (6, 6);

// class RotationError(Exception):
//     pass

struct BasePiece {
    piece: Array2<String>,
    board_pos_row: usize,
    board_pos_col: usize,
}

impl BasePiece {
    fn new(piece: Array2<String>) -> Self {
        BasePiece {
            piece,
            board_pos_row: 0,
            board_pos_col: 0,
        }
    }

    fn move_to(&mut self, row: usize, col: usize) -> bool {
        // Since row is a "usize" we know it can never be below 0
        if row >= GRID_SIZE.0 || col >= GRID_SIZE.1 {
            return false;
        }
        self.board_pos_row = row;
        self.board_pos_col = col;
        true
    }

    fn shift_left(&mut self) -> bool {
        if self.board_pos_col == 0 {
            return false;
        }
        self.move_to(self.board_pos_row, self.board_pos_col - 1)
    }

    fn shift_right(&mut self) -> bool {
        self.move_to(self.board_pos_row, self.board_pos_col + 1)
    }

    fn shift_up(&mut self) -> bool {
        if self.board_pos_row == 0 {
            return false;
        }
        self.move_to(self.board_pos_row - 1, self.board_pos_col)
    }

    fn shift_down(&mut self) -> bool {
        self.move_to(self.board_pos_row + 1, self.board_pos_col)
    }

    fn rotate(&mut self) -> bool {
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
        true
    } 

}

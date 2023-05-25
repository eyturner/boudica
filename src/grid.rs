use crate::grid::hex::Hex;
use crate::grid::piece::Piece;
use std::collections::HashMap;
use std::fmt;

pub mod hex;
pub mod piece;

#[derive(Debug)]
pub struct Grid {
    grid: HashMap<Hex, Piece>,
    p1_hand: Vec<Piece>,
    p2_hand: Vec<Piece>,
}

impl Grid {
    pub fn new() -> Grid {
        return Grid {
            grid: HashMap::new(),
            p1_hand: Vec::new(),
            p2_hand: Vec::new(),
        };
    }

    pub fn export_ign(&self) -> &str {
        return "wa1";
    }

    pub fn print_ascii(&self) {
        println!("ASCII HERE!!");
        // Code to print an ascii version of the current grid
    }

    fn make_move(&self, piece: Piece, prev_hex: Hex, new_hex: Hex) {
        // update board to reflect move
    }

    fn add_to_grid(&self, new_piece: Piece, hex: Hex) {
        // Update board to reflect piece added
    }
}

impl fmt::Display for Grid {
    !unimplemented!("Use the print ASCII function here");
}

// IGN: Inline Grid Notation
fn import_from_ign(fen: &str) -> Grid {
    // Pick an orientation of the board such that the top of each piece is flat.

    // find the column that is the left-most and the piece at the top of that column
    // you will be reading up --> down and then left --> right

    return Grid::new();
}

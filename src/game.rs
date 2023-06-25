use crate::game::hex::Hex;
use crate::game::piece::Piece;
use petgraph::algo;
use petgraph::Graph;
use std::fmt;

pub mod hex;
pub mod piece;

#[derive(Debug)]
pub enum HexEdge {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug, Default)]
pub struct Game {
    grid: Graph<Piece, HexEdge, petgraph::Undirected>,
    p1_hand: Vec<Piece>,
    p2_hand: Vec<Piece>,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            grid: Graph::<Piece, HexEdge, petgraph::Undirected>::new_undirected(),
            p1_hand: Vec::new(),
            p2_hand: Vec::new(),
        };
    }

    pub fn export_ign(&self) -> &str {
        return "wa1";
    }

    fn make_move(&self, piece: Piece, prev_hex: Hex, new_hex: Hex) {
        // update board to reflect move
    }

    fn add_to_grid(&self, new_piece: Piece, hex: Hex) {
        // Update board to reflect piece added
    }
}

// IGN: Inline Grid Notation
fn import_from_ign(fen: &str) -> Game {
    // Pick an orientation of the board such that the top of each piece is flat.

    // find the column that is the left-most and the piece at the top of that column
    // you will be reading up --> down and then left --> right

    return Game::new();
}

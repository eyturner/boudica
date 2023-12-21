use crate::game::hex::Hex;
use crate::game::piece::{Piece, PieceMove, PieceType};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use self::piece::PieceColor;

pub mod hex;
pub mod piece;

#[derive(Debug, PartialEq, Eq)]
pub enum HexEdge {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
    T,
    B,
}

pub fn get_starting_hand(color: PieceColor) -> Vec<Piece> {
    let mut color_prefix = String::from("b");
    if &color == &PieceColor::White {
        color_prefix = String::from("w");
    }
    return vec![
        Piece {
            color,
            piece_type: PieceType::QueenBee,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "q",
        },
        Piece {
            color,
            piece_type: PieceType::Spider,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "s1",
        },
        Piece {
            color,
            piece_type: PieceType::Spider,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "s2",
        },
        Piece {
            color,
            piece_type: PieceType::Beetle,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "b1",
        },
        Piece {
            color,
            piece_type: PieceType::Beetle,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "b2",
        },
        Piece {
            color,
            piece_type: PieceType::Grasshopper,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "g1",
        },
        Piece {
            color,
            piece_type: PieceType::Grasshopper,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "g2",
        },
        Piece {
            color,
            piece_type: PieceType::Grasshopper,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "g3",
        },
        Piece {
            color,
            piece_type: PieceType::Ant,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "a1",
        },
        Piece {
            color,
            piece_type: PieceType::Ant,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "a2",
        },
        Piece {
            color,
            piece_type: PieceType::Ant,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "a3",
        },
        Piece {
            color,
            piece_type: PieceType::Mosquito,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "m",
        },
        Piece {
            color,
            piece_type: PieceType::Ladybug,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "l",
        },
        Piece {
            color,
            piece_type: PieceType::Pillbug,
            hex: Hex::new(),
            in_hand: true,
            name: color_prefix.clone() + "p",
        },
    ];
}

#[derive(Debug, Default)]
pub struct Game {
    pub grid: Graph<Piece, HexEdge, petgraph::Undirected>,
    pub p1_hand: Vec<Piece>,
    pub p2_hand: Vec<Piece>,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            grid: Graph::<Piece, HexEdge, petgraph::Undirected>::new_undirected(),
            p1_hand: get_starting_hand(PieceColor::Black),
            p2_hand: get_starting_hand(PieceColor::White),
        };
    }

    pub fn export_ign(&self) -> &str {
        todo!();
    }

    pub fn make_move(&mut self, piece_move: PieceMove) {
        if let Some(fp) = self
            .grid
            .node_weights_mut()
            .find(|p| p.name == piece_move.piece.name)
        {
            // TODO: Update old edges
            fp.hex = piece_move.hex;

            // TODO: Update new edges
        } else {
            eprintln!("Unable to find specified piece in make_move");
        }
    }

    pub fn add_to_grid(&mut self, mut piece_placement: PieceMove) -> NodeIndex {
        // Remove piece from both hands:
        self.p1_hand.retain(|p| *p != piece_placement.piece);
        self.p2_hand.retain(|p| *p != piece_placement.piece);

        // Update new_piece
        piece_placement.piece.in_hand = false;
        piece_placement.piece.hex = piece_placement.hex;

        // TODO: Update board to reflect piece added
        let new_piece = self.grid.add_node(piece_placement.piece);

        // TODO: Update Edges for new_piece
        //

        return new_piece;
    }
}

// IGN: Inline Grid Notation
// Example IGN: wa1 ba1 1_bb1 |1| wb1 wq ba2 bq ...

pub fn import_from_ign(ign: &str) -> Game {
    // Pick an orientation of the board such that the top of each piece is flat.
    // find the column that is the left-most and the piece at the top of that column
    // you will be reading up --> down and then left --> right

    todo!();
    // let rows = ign.lines();
    // let mut g = Game::new();
    //
    // return Game::new();
}

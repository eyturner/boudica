use super::hex::Hex;
use crate::game::HexEdge;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    QueenBee,
    Ant,
    Beetle,
    Grasshopper,
    Ladybug,
    Spider,
    Mosquito,
    Pillbug,
}

#[derive(Debug)]
pub struct PieceMove {
    pub piece_node: NodeIndex,
    pub hex: Hex,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub name: String,
    pub hex: Hex,
    pub in_hand: bool,
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color && self.piece_type == other.piece_type;
    }
}

impl Piece {
    pub fn can_move(&self, grid: &Graph<Piece, HexEdge, petgraph::Undirected>) -> bool {
        todo!();
    }

    pub fn new(color: PieceColor, piece_type: PieceType, name_suffix: &str) -> Self {
        let mut color_prefix = String::from("b");
        if &color == &PieceColor::White {
            color_prefix = String::from("w");
        }

        return Piece {
            color,
            hex: Hex::new(),
            piece_type,
            name: color_prefix.clone() + name_suffix,
            in_hand: true,
        };
    }

    // pub fn get_moves(&self, game: Game) -> Vec<PieceMove> {
    //     match self.piece_type {
    //         PieceType::QueenBee => {
    //             if self.can_move() {
    //                 todo!();
    //             }
    //         }
    //     }
    // }
}

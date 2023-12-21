use super::hex::Hex;
use super::Game;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Eq, PartialEq)]
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
    pub piece: Piece,
    pub hex: Hex,
}

#[derive(Debug)]
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
    pub fn can_move(&self) -> bool {
        todo!();
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

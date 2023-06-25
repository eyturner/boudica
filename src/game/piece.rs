use super::hex::Hex;

#[derive(Debug)]
enum PieceType {
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
pub struct Piece {
    color: String,
    piece_type: PieceType,
    name: String,
    hex: Hex,
}

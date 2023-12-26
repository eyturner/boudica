use crate::game::hex::{Hex, HexEdge};
use crate::game::piece::{Piece, PieceMove, PieceType};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use self::hex::get_edge_types;
use self::piece::PieceColor;

pub mod hex;
pub mod piece;

pub fn get_starting_hand(color: PieceColor) -> Vec<Piece> {
    return vec![
        Piece::new(color, PieceType::QueenBee, "q"),
        Piece::new(color, PieceType::Spider, "s1"),
        Piece::new(color, PieceType::Spider, "s2"),
        Piece::new(color, PieceType::Beetle, "b1"),
        Piece::new(color, PieceType::Beetle, "b2"),
        Piece::new(color, PieceType::Grasshopper, "g1"),
        Piece::new(color, PieceType::Grasshopper, "g2"),
        Piece::new(color, PieceType::Grasshopper, "g3"),
        Piece::new(color, PieceType::Ant, "a1"),
        Piece::new(color, PieceType::Ant, "a2"),
        Piece::new(color, PieceType::Ant, "a3"),
        Piece::new(color, PieceType::Mosquito, "m"),
        Piece::new(color, PieceType::Ladybug, "l"),
        Piece::new(color, PieceType::Pillbug, "p"),
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

    pub fn get_all_moves(&self) -> Vec<PieceMove> {
        let mut valid_moves: Vec<PieceMove> = Vec::new();
        for piece in self.grid.node_weights() {
            if piece.can_move(&self.grid) {
                for pm in piece.get_moves(&self) {
                    valid_moves.push(pm);
                }
            }
        }
        return valid_moves;
    }

    /// Assumes piece_move is valid move
    pub fn make_move(&mut self, piece_move: PieceMove) {
        // get piece from node:
        let piece = self
            .grid
            .node_weight(piece_move.piece_node)
            .expect("Unable to find Piece")
            .clone();
        self.grid.remove_node(piece_move.piece_node);
        self.add_to_grid(piece, piece_move.hex);
    }

    pub fn umake_move(&mut self, piece_move: PieceMove) {
        // piece_move should contain the previous location of the piece
        todo!();
    }

    pub fn add_to_grid(&mut self, mut piece: Piece, hex: Hex) -> NodeIndex {
        // Remove piece from both hands:
        self.p1_hand.retain(|p| *p != piece);
        self.p2_hand.retain(|p| *p != piece);

        // Update new_piece
        piece.in_hand = false;
        piece.hex = hex;

        let new_piece = self.grid.add_node(piece);

        self.update_piece_edges(new_piece);

        return new_piece;
    }

    pub fn update_piece_edges(&mut self, piece: NodeIndex) {
        for hex_edge in get_edge_types() {
            let piece_node = self
                .grid
                .node_indices()
                .find(|&p| self.grid[p].hex == self.grid[piece].hex)
                .expect("Error finding piece_node in game_grid");

            let neighbor_hex = self.grid[piece].hex.get_neighbor(hex_edge);

            if let Some(found_neighbor_p) = self
                .grid
                .node_indices()
                .find(|&p| self.grid[p].hex == neighbor_hex)
            {
                // Add connection both ways
                self.grid.add_edge(piece_node, found_neighbor_p, hex_edge);
                self.grid
                    .add_edge(found_neighbor_p, piece_node, hex_edge.get_opposite());
            }
        }
    }
}

// Returns true if hex is attached to some piece in game.grid other than itself
fn hex_is_connected(hex: Hex, game: &Game, piece_id: &str) -> bool {
    if let Some(_neighbor) = game
        .grid
        .node_weights()
        .find(|&piece| piece.hex.get_neighbors().contains(&hex) && piece.id != piece_id)
    {
        return true;
    }
    return false;
}

// IGN: Inline Grid Notation
// Example IGN: wa1 ba1 1_bb1 |1| wb1 wq ba2 bq ...

pub fn import_from_ign(ign: &str) -> Game {
    // Pick an orientation of the board such that the top of each piece is flat.
    // find the column that is the left-most and the piece at the top of that column
    // you will be reading up --> down and then left --> right

    todo!();
}

use crate::game::hex::{Hex, HexEdge};
use crate::game::piece::{Piece, PieceType};
use crate::game::piece_move::PieceMove;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use self::hex::get_edge_types;
use self::piece::PieceColor;

pub mod hex;
pub mod piece;
pub mod piece_move;

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
    pub move_list: Vec<PieceMove>,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            grid: Graph::<Piece, HexEdge, petgraph::Undirected>::new_undirected(),
            p1_hand: get_starting_hand(PieceColor::Black),
            p2_hand: get_starting_hand(PieceColor::White),
            move_list: Vec::new(),
        };
    }

    pub fn export_ign(&self) -> &str {
        todo!();
    }

    pub fn get_all_moves(&self) -> Vec<PieceMove> {
        let mut valid_moves: Vec<PieceMove> = Vec::new();
        for piece in self.grid.node_weights() {
            if piece.can_move(self) {
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
        self.move_list.push(piece_move);
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

    pub fn hex_connects(&self, hex: Hex) -> bool {
        if let Some(_neighbor) = self
            .grid
            .node_weights()
            .find(|&piece| piece.hex.get_neighbors().contains(&hex))
        {
            return true;
        }
        return false;
    }

    pub fn slide_distance(&self, start: Hex, end: Hex) -> usize {
        println!("Looking for dist between {:?} and {:?}", start, end);
        // Let's do some BFS, baby!
        let mut total_distance: usize = 1;
        let mut hexes_to_check: Vec<Hex> = start.get_neighbors();

        // Only keep hexes that dont contain pieces but are still connected to the grid
        hexes_to_check.retain(|h| {
            self.hex_connects(*h) && self.grid.node_weights().find(|&p| p.hex == *h).is_none()
        });

        while !hexes_to_check.is_empty() {
            let mut next_hexes: Vec<Hex> = Vec::new();
            while let Some(curr_hex) = hexes_to_check.pop() {
                if curr_hex == end {
                    return total_distance;
                }
                next_hexes.extend(curr_hex.get_neighbors())
            }

            // No more hexes in hexes_to_check -> now go through next_hexes and add 1 to
            // distance
            hexes_to_check = next_hexes;
            total_distance += 1;
        }
        // This means that there is no path between start and end, which should be impossible.
        unreachable!()
    }

    pub fn grid_distance(&self, start: Hex, end: Hex) -> usize {
        // Returns distance along grid
        return (start.q.abs_diff(end.q) + start.r.abs_diff(end.r) + start.s.abs_diff(end.s)) / 2;
    }
}

// Returns true if hex is attached to some piece in game.grid other than itself
fn piece_is_connected(hex: Hex, game: &Game, piece_id: &str) -> bool {
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

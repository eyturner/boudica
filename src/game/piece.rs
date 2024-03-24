use super::hex::Hex;
use super::piece_move::*;
use crate::game::{Game, HexEdge};

use petgraph::algo::dijkstra;

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
    Spider,
    Ladybug,
    Pillbug,
    Mosquito,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub id: String,
    pub hex: Hex,
    pub in_hand: bool,
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color && self.piece_type == other.piece_type;
    }
}

impl Piece {
    pub fn new(color: PieceColor, piece_type: PieceType, name_suffix: &str) -> Self {
        let mut color_prefix = String::from("b");
        if &color == &PieceColor::White {
            color_prefix = String::from("w");
        }

        return Piece {
            color,
            hex: Hex::new(),
            piece_type,
            id: color_prefix.clone() + name_suffix,
            in_hand: true,
        };
    }

    pub fn can_move(&self, game: &Game) -> bool {
        // Dijkstra's algo to check if graph is fully connected
        if let Some(piece_node) = game
            .grid
            .node_indices()
            .find(|&n| game.grid[n].id == self.id)
        {
            // Remove the node, and compare number of nodes to number of paths
            let mut grid_clone = game.grid.clone();
            if let Some(_) = grid_clone.remove_node(piece_node) {
                let paths = dijkstra(
                    &grid_clone,
                    grid_clone
                        .node_indices()
                        .next()
                        .expect("No items in the graph"),
                    None,
                    |_| 1,
                );
                return paths.len() == grid_clone.node_count();
                // grid_clone is dropped here
            }
        }

        // TODO: Check for previous PillBug move
        return false;
    }

    pub fn get_moves(&self, game: &Game) -> Vec<PieceMove> {
        let mut valid_moves: Vec<PieceMove> = Vec::new();
        if let Some(curr_piece_node) = game
            .grid
            .node_indices()
            .find(|&n| game.grid[n].id == self.id)
        {
            match self.piece_type {
                PieceType::QueenBee => {
                    valid_moves.extend(get_queen_moves(self, curr_piece_node, game));
                }
                PieceType::Ant => {
                    valid_moves.extend(get_ant_moves(self, curr_piece_node, game));
                }
                PieceType::Beetle => {
                    valid_moves.extend(get_beetle_moves(self, curr_piece_node, game));
                }
                PieceType::Grasshopper => {
                    valid_moves.extend(get_grasshopper_moves(self, curr_piece_node, game));
                }
                PieceType::Spider => {
                    valid_moves.extend(get_spider_moves(self, curr_piece_node, game));
                }
                PieceType::Ladybug => {
                    valid_moves.extend(get_ladybug_moves(self, curr_piece_node, game));
                }
                PieceType::Pillbug => {
                    valid_moves.extend(get_pillbug_moves(self, curr_piece_node, game));
                }
                PieceType::Mosquito => {
                    valid_moves.extend(get_mosquito_moves(self, curr_piece_node, game));
                }
            }
        }
        return valid_moves;
    }
}

pub fn can_slide(hex: Hex, dir: HexEdge, game: &Game) -> bool {
    // No piece alrady there:
    if let Some(_) = game
        .grid
        .node_weights()
        .find(|&p| p.hex == hex.get_neighbor(dir))
    {
        return false;
    }

    // No gate blocking slide
    let gate_blockers = dir.get_gate_edges().map(|e| hex.get_neighbor(e));
    return game
        .grid
        .node_weights()
        .filter(|&p| gate_blockers.contains(&p.hex))
        .count()
        < 2;
}

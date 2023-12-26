use super::hex::{get_slide_edge_types, Hex};
use crate::game::{Game, HexEdge};
use petgraph::algo::dijkstra;
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

    pub fn can_move(&self, grid: &Graph<Piece, HexEdge, petgraph::Undirected>) -> bool {
        // Dijkstra's algo to check if graph is fully connected
        if let Some(piece_node) = grid.node_indices().find(|&n| grid[n].id == self.id) {
            // Remove the node, and compare number of nodes to number of paths
            let mut grid_clone = grid.clone();
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
        return false;
    }

    pub fn get_moves(&self, game: &Game) -> Vec<PieceMove> {
        let mut valid_moves: Vec<PieceMove> = Vec::new();
        // println!(
        //     "{:?}, {:?}",
        //     game.grid
        //         .node_indices()
        //         .find(|&n| game.grid[n].id == self.id)
        //         .expect("Unable to find :("),
        //     self
        // );
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
                _ => {
                    return todo!();
                }
            }
        }
        return valid_moves;
    }
}

pub fn get_queen_moves(queen: &Piece, queen_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if queen.can_move(&game.grid) {
        let queen_neighbor_edges = get_slide_edge_types();
        for e in queen_neighbor_edges {
            if can_slide(queen.hex, e, game) {
                valid_moves.push(PieceMove {
                    piece_node: queen_node,
                    hex: queen.hex.get_neighbor(e),
                })
            }
        }
    }
    return valid_moves;
}

pub fn get_ant_moves(ant: &Piece, ant_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    // Returns true if hex is attached to some piece in game.grid other than itself
    fn hex_is_connected(hex: Hex, game: &Game, ant_id: &str) -> bool {
        if let Some(_neighbor) = game
            .grid
            .node_weights()
            .find(|&piece| piece.hex.get_neighbors().contains(&hex) && piece.id != ant_id)
        {
            return true;
        }
        return false;
    }

    let mut valid_moves: Vec<PieceMove> = Vec::new();
    // Confirm Ant is not pinned:
    if ant.can_move(&game.grid) {
        // BFS using a queue to determine all the hexes the ant can move to:
        let mut hexes_to_check: Vec<Hex> = ant.hex.get_slide_neighbors();
        hexes_to_check.retain(|&n| {
            game.grid.node_weights().find(|p| p.hex == n).is_none()
                && hex_is_connected(n, game, &ant.id)
        });

        while !hexes_to_check.is_empty() {
            // If game doesn't have piece at h, add to valid_moves,
            if let Some(h) = hexes_to_check.pop() {
                if game.grid.node_weights().find(|&p| p.hex == h).is_none()
                    && valid_moves.iter().find(|&m| m.hex == h).is_none()
                {
                    valid_moves.push(PieceMove {
                        piece_node: ant_node,
                        hex: h,
                    });
                    // Check all neighbors for h as well.
                    let mut h_neighbors = h.get_neighbors();
                    h_neighbors.retain(|&n| {
                        !game.grid.node_weights().find(|p| p.hex == n).is_some()
                            && !valid_moves.iter().find(|m| m.hex == n).is_some()
                            && hex_is_connected(n, game, &ant.id)
                    });
                    hexes_to_check.extend(h_neighbors);
                }
            }
        }
    }
    return valid_moves;
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

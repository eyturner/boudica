use super::hex::{get_edge_types, Hex};
use crate::game::{hex, piece_is_connected, Game, HexEdge};
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
    Spider,
    Ladybug,
    Pillbug,
    Mosquito,
}

#[derive(Debug, Eq, PartialEq)]
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

pub fn get_queen_moves(queen: &Piece, queen_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if queen.can_move(&game) {
        let queen_neighbor_edges = get_edge_types();
        for e in queen_neighbor_edges {
            if can_slide(queen.hex, e, game)
                && piece_is_connected(queen.hex.get_neighbor(e), game, &queen.id)
            {
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
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    // Confirm Ant is not pinned:
    if ant.can_move(&game) {
        // BFS using a queue to determine all the hexes the ant can move to:
        let mut hexes_to_check: Vec<Hex> = ant.hex.get_neighbors();
        hexes_to_check.retain(|&n| {
            game.grid.node_weights().find(|p| p.hex == n).is_none()
                && piece_is_connected(n, game, &ant.id)
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
                            && piece_is_connected(n, game, &ant.id)
                    });
                    hexes_to_check.extend(h_neighbors);
                }
            }
        }
    }
    return valid_moves;
}

pub fn get_beetle_moves(beetle: &Piece, beetle_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if beetle.can_move(&game) {
        let beetle_neighbor_edges = get_edge_types();
        for e in beetle_neighbor_edges {
            if can_slide(beetle.hex, e, game)
                && piece_is_connected(beetle.hex.get_neighbor(e), game, &beetle.id)
            {
                valid_moves.push(PieceMove {
                    piece_node: beetle_node,
                    hex: beetle.hex.get_neighbor(e),
                })
            } else if let Some(bottom_piece) = game
                .grid
                .node_weights()
                .find(|&piece| piece.hex == beetle.hex.get_neighbor(e))
            {
                // Add move to place beetle on top!
                valid_moves.push(PieceMove {
                    piece_node: beetle_node,
                    hex: Hex {
                        q: bottom_piece.hex.q,
                        r: bottom_piece.hex.r,
                        s: bottom_piece.hex.s,
                        z: bottom_piece.hex.z + 1,
                    },
                })
            }
        }
    }
    return valid_moves;
}

pub fn get_grasshopper_moves(
    grasshopper: &Piece,
    grasshopper_node: NodeIndex,
    game: &Game,
) -> Vec<PieceMove> {
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if grasshopper.can_move(&game) {
        let jump_dirs = get_edge_types();
        for dir in jump_dirs {
            println!("On {:?}", dir);
            let mut jumps: usize = 0;
            let mut jump_hex = grasshopper.hex.clone();
            while let Some(curr_piece) = game
                .grid
                .node_weights()
                .find(|&piece| piece.hex == jump_hex.get_neighbor(dir))
            {
                jumps += 1;
                jump_hex = curr_piece.hex.get_neighbor(dir);
            }

            // Add jump now (if we've done at least 1 move)
            if jumps > 0 {
                valid_moves.push(PieceMove {
                    piece_node: grasshopper_node,
                    hex: jump_hex,
                });
            }
        }
    }
    return valid_moves;
}

pub fn get_spider_moves(spider: &Piece, spider_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    let spider_move_distance = 3;
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if spider.can_move(&game) {
        // BFS using a queue to determine all the hexes the spider can move to:
        let mut hexes_to_check: Vec<Hex> = spider.hex.get_neighbors();
        let mut checked_hexes: Vec<Hex> = Vec::new();
        hexes_to_check.retain(|&n| {
            game.grid.node_weights().find(|p| p.hex == n).is_none() // No piece at n
                && piece_is_connected(n, game, &spider.id)
        });

        while !hexes_to_check.is_empty() {
            // If game doesn't have piece at h, add to valid_moves,
            if let Some(h) = hexes_to_check.pop() {
                if game.grid.node_weights().find(|&p| p.hex == h).is_none() // No piece at h
                // Haven't checked this
                // hex yet
                    && checked_hexes.iter().find(|&prev_h| prev_h == &h).is_none()
                {
                    if game.slide_distance(spider.hex, h) == spider_move_distance {
                        valid_moves.push(PieceMove {
                            piece_node: spider_node,
                            hex: h,
                        });
                    } else {
                        // Check all neighbors for h as well.
                        let mut h_neighbors = h.get_neighbors();
                        h_neighbors.retain(|&n| {
                            game.grid.node_weights().find(|p| p.hex == n).is_none() // n is open
                                && valid_moves.iter().find(|m| m.hex == n).is_none()
                                && checked_hexes.iter().find(|&prev_h| prev_h == &n).is_none() // haven't
                                                                                                // checked n yet
                                && piece_is_connected(n, game, &spider.id) // n is connected to some
                                                                           // other piece in the grid
                        });
                        hexes_to_check.extend(h_neighbors);
                    }
                }
                // Add hex to checked hexes so we don't check it again
                checked_hexes.push(h);
            }
        }
    }
    return valid_moves;
}

pub fn get_ladybug_moves(ladybug: &Piece, ladybug_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    let lb_top_distance = 2;
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    if ladybug.can_move(&game) {
        // BFS using a queue to determine all the hexes the spider can move to:
        let mut hexes_to_check: Vec<Hex> = ladybug.hex.get_neighbors();
        let mut final_top_hexes: Vec<Hex> = Vec::new();
        let mut top_distance: usize = 1;
        hexes_to_check.retain(|&n| {
            game.grid.node_weights().find(|p| p.hex == n).is_some() // Piece at n
                && piece_is_connected(n, game, &ladybug.id)
        });

        // Across the top of the hive:
        while !hexes_to_check.is_empty() {
            // If game has piece at h, check distance and add to final_top_hexes + checked_hexes,
            if let Some(h) = hexes_to_check.pop() {
                if game.grid.node_weights().find(|&p| p.hex == h).is_some()
                // Piece at h
                // Haven't checked this
                // hex yet
                    && final_top_hexes.iter().find(|&prev_h| prev_h == &h).is_none()
                {
                    if top_distance == lb_top_distance {
                        final_top_hexes.push(h);
                    } else {
                        // Check all neighbors for h as well.
                        let mut h_neighbors = h.get_neighbors();
                        h_neighbors.retain(|&n| {
                            game.grid.node_weights().find(|p| p.hex == n).is_some()
                            // n has a top
                        });
                        hexes_to_check.extend(h_neighbors);
                        // Increase distance on top by 1
                        top_distance += 1;
                    }
                }
            }
        }

        // Finally, which steps down are available
        while let Some(top_h) = final_top_hexes.pop() {
            let mut h_neighbors = top_h.get_neighbors();
            h_neighbors.retain(|&n| {
                game.grid.node_weights().find(|p| p.hex == n).is_none() // no piece at n
                && valid_moves.iter().find(|&m| m.hex == n).is_none()
            });
            valid_moves.extend(h_neighbors.iter().map(|&hn| PieceMove {
                piece_node: ladybug_node,
                hex: hn,
            }));
        }
    }
    return valid_moves;
}

pub fn get_pillbug_moves(pillbug: &Piece, pillbug_node: NodeIndex, game: &Game) -> Vec<PieceMove> {
    // Start with queen-type moves
    let mut valid_moves: Vec<PieceMove> = get_queen_moves(pillbug, pillbug_node, game);

    // If space around pillbug, add all mobile pieces to nearby spaces:
    let mut empty_neighbors = pillbug.hex.get_neighbors();
    empty_neighbors.retain(|&n| game.grid.node_weights().find(|&p| p.hex == n).is_none());

    let mut mobile_pieces_hexes = pillbug.hex.get_neighbors();
    mobile_pieces_hexes.retain(|&n| {
        game.grid
            .node_weights()
            .find(|&p| p.hex == n)
            .is_some_and(|p| p.can_move(game))
    });

    while let Some(open_space) = empty_neighbors.pop() {
        for mph in &mobile_pieces_hexes {
            valid_moves.push(PieceMove {
                piece_node: game
                    .grid
                    .node_indices()
                    .find(|&p| game.grid[p].hex == *mph)
                    .expect("Unable to unwrap piece_node"),
                hex: open_space,
            });
        }
    }

    return valid_moves;
}

pub fn get_mosquito_moves(
    mosquito: &Piece,
    mosquito_node: NodeIndex,
    game: &Game,
) -> Vec<PieceMove> {
    let mut valid_moves: Vec<PieceMove> = Vec::new();
    let neighbor_hexes = mosquito.hex.get_neighbors();

    for hex in neighbor_hexes {
        if let Some(neighbor_piece) = game.grid.node_weights().find(|&p| p.hex == hex) {
            match neighbor_piece.piece_type {
                PieceType::QueenBee => {
                    let possible_moves = get_queen_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Ant => {
                    let possible_moves = get_ant_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Beetle => {
                    let possible_moves = get_beetle_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Grasshopper => {
                    let possible_moves = get_grasshopper_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Spider => {
                    let possible_moves = get_spider_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Ladybug => {
                    let possible_moves = get_ladybug_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Pillbug => {
                    let possible_moves = get_pillbug_moves(mosquito, mosquito_node, game);
                    for m in possible_moves {
                        if valid_moves
                            .iter()
                            .find(|&found_move| found_move == &m)
                            .is_none()
                        {
                            valid_moves.push(m);
                        }
                    }
                }
                PieceType::Mosquito => {
                    valid_moves.extend([]);
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

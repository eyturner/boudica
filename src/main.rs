use crate::game::{
    hex::Hex,
    piece::{Piece, PieceColor, PieceType},
};

mod game;

// pub struct GameLog {
//     turn: u32,
//     moves: Vec<PieceMove>,
// }

fn main() {
    let mut game = game::Game::new();
    // let game_log = GameLog {
    //     turn: 0,
    //     moves: Vec::new(),
    // };

    let first_piece = Piece::new(PieceColor::Black, PieceType::QueenBee, "q");
    let second_piece = Piece::new(PieceColor::White, PieceType::QueenBee, "q");
    let third_piece = Piece::new(PieceColor::White, PieceType::Grasshopper, "g1");
    let cuarto_piece = Piece::new(PieceColor::Black, PieceType::Pillbug, "p");

    let fmi = game.add_to_grid(
        first_piece,
        Hex {
            q: 0,
            r: 0,
            s: 0,
            z: 0,
        },
    );
    let smi = game.add_to_grid(
        second_piece,
        Hex {
            q: 0,
            r: -1,
            s: 1,
            z: 0,
        },
    );

    let tmi = game.add_to_grid(
        third_piece,
        Hex {
            q: -1,
            r: 0,
            s: 1,
            z: 0,
        },
    );

    let cmi = game.add_to_grid(
        cuarto_piece,
        Hex {
            q: -1,
            r: -1,
            s: 2,
            z: 0,
        },
    );

    let legal_moves = game.get_all_moves();

    for m in legal_moves {
        println!("{:?}", m);
    }

    /*
     * Command line args to determine if the players are users or cpus
     *
     * Main game loop for CPUs:
     *
     *  1. input current board arrangement
     *      need to support input from inline notation -> fns: from_ign, to_ign -> convert to graph
     *
     *  2. use minimax with weights to determine best move for current player with current grid
     *      fns: generate_moves, calc_score
     *
     *  3. make move
     *      fns: update_graph, update_hands
     *
     * */
}

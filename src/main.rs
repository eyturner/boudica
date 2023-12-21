use crate::game::{
    hex::Hex,
    piece::{Piece, PieceColor, PieceMove, PieceType},
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
    let first_move = PieceMove {
        piece: Piece {
            name: String::from("bq"),
            color: PieceColor::Black,
            hex: Hex {
                q: 0,
                r: 0,
                s: 0,
                z: 2,
            },
            piece_type: PieceType::QueenBee,
            in_hand: true,
        },
        hex: Hex {
            q: 0,
            r: 0,
            s: 0,
            z: 0,
        },
    };
    game.add_to_grid(first_move);

    let test_piece = Piece {
        name: String::from("bq"),
        color: PieceColor::Black,
        hex: Hex {
            q: 0,
            r: 0,
            s: 0,
            z: 0,
        },
        piece_type: PieceType::QueenBee,
        in_hand: false,
    };

    let test_move = PieceMove {
        piece: test_piece,
        hex: Hex {
            q: 1,
            r: 1,
            s: 1,
            z: 1,
        },
    };

    game.make_move(test_move);

    for piece in game.p1_hand {
        println!("{:?}", piece);
    }

    println!("\n\n{:?}", game.grid);

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

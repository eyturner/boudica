mod game;

fn main() {
    let mut game = game::Game::new();
    println!("We've got this working: {:?}", game);

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

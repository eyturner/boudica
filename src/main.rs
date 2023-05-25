mod grid;

fn main() {
    let mut game_grid = grid::Grid::new();
    game_grid.print_ascii();
    println!("We've got this working: {:?}", game_grid);
}

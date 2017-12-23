extern crate generic_array;

mod game;
mod tictactoe;
mod agents;

use game::boardgame::WinResult;
use tictactoe::tictactoe::TicTacToe;
use agents::random_agent::RandomAgent;

fn main() {
    println!("Hello, world!");

    let tictactoe = TicTacToe{};

    let white_player = RandomAgent{};
    let black_player = RandomAgent{};

    game::runner::run_game(tictactoe, white_player, black_player);
}

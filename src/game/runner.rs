use game::boardgame::BoardGame;
use game::game_agent::GameAgent;
use game::game_rules::GameRules;
use game::board::Piece;
use agents::random_agent::RandomAgent;

pub fn run_game<T: GameRules + Default + Clone, B: GameAgent, W: GameAgent>(
    game: BoardGame<T, B, W>,
) {

    //game.reset();

    // loop {
    //     let player_turn = game.get_player_turn();

    //     let boardstate = game.get_boardstate();
    //     let legal_moves = game.legal_moves(player_turn);

    //     let player_agent = if player_turn == BoardGame::Color::Black {
    //         &white
    //     }
    //     else {
    //         &black
    //     };

    // }
}

mod tests {
    use game::boardgame::BoardGame;
    use game::game_rules::PlayerColor;
    use tictactoe::tictactoe::TicTacToe;

    #[test]
    fn can_play_simple_game() {
        let white = RandomAgent::new();
        let black = RandomAgent::new();
        let game = BoardGame::<TicTacToe>::new(black, white);

        game.reset();

        loop {
            let player_turn = game.get_player_turn();
            let legal_moves = game.legal_moves(player_turn);

            let player_agent = match player_turn {
                BoardGame::PlayerColor::Black => &black,
                BoardGame::PlayerColor::White => &white,
                _ => panic!("player must be black or white"),
            };

            let boardstate = game.boardstate();
            let picked_move = player_agent.pick_move(&boardstate);

            game.apply_move(picked_move);

            if let Some(winner) = game.winner() {
                println!("game over!");
                break;
            }
        }
    }
}

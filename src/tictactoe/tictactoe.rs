use game::game_rules::{GameMove, GameResult, GameRules, PlayerColor};
use game::board::BoardState;

#[derive(Clone)]
pub struct TicTacToe;

#[derive(Clone)]
pub enum TicTacToePiece {}

impl GameRules for TicTacToe {
    type Piece = TicTacToePiece;

    fn is_game_over(board: &BoardState<Self::Piece>) -> bool {
        true
    }

    fn winner(board: &BoardState<Self::Piece>) -> GameResult {
        GameResult::Win(PlayerColor::Black)
    }

    fn legal_moves(
        board: &BoardState<Self::Piece>,
        player_color: PlayerColor,
    ) -> Vec<GameMove<Self::Piece>> {
        vec![]
    }

    fn next_state(board: &BoardState<Self::Piece>) -> BoardState<Self::Piece> {
        board.clone()
    }
}

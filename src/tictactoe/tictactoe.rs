use game::game_rules::GameRules;

pub struct TicTacToe;

pub enum TicTacToePiece {}

impl GameRules for TicTacToe {
    type Piece = TicTacToePiece;
}

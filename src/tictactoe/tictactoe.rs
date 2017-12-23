use game::boardgame::BoardGame;

pub struct TicTacToe;

pub struct TicTacToePiece;

impl BoardGame<TicTacToePiece> for TicTacToe {

fn winner(&self) -> Option<WinResult> {

}

fn take_turn(&self) {

}

fn whose_turn(&self) -> PlayerColor {

}

fn get_boardstate(&self) -> &BoardState<TicTacToePiece> {

}
}
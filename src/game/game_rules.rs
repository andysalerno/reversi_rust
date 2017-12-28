use game::board::{BoardState, Piece};

#[derive(Copy, Clone)]
pub enum PlayerColor {
    Black,
    White,
}

pub struct GameMove<T> {
    piece: Piece<T>,
    pos: (usize, usize),
}

#[derive(Copy, Clone)]
pub enum GameResult {
    Win(PlayerColor),
    Tie,
}

pub trait GameRules {
    type Piece: Clone;

    fn is_game_over(board: &BoardState<Self::Piece>) -> bool;

    fn winner(board: &BoardState<Self::Piece>) -> GameResult;

    fn legal_moves(
        board: &BoardState<Self::Piece>,
        player_color: PlayerColor,
    ) -> Vec<GameMove<Self::Piece>>;

    fn next_state(board: &BoardState<Self::Piece>) -> BoardState<Self::Piece>;
}

mod tests {
    use game::board::BoardState;
    use tictactoe::tictactoe::{TicTacToe, TicTacToePiece};
    use game::game_rules::PlayerColor;

    #[test]
    fn simple_gamerules_impl() {
        let rules = TicTacToe {};
        let board = BoardState::<TicTacToePiece>::new();

        let is_game_over = rules.is_game_over(&board);

        let winner = rules.winner(&board);

        let legal_moves = rules.legal_moves(&board, PlayerColor::Black);

        let game_move = GameMove {};
        let next_state = rules.next_state(&board, game_move);
    }
}

use game::board::{ Piece, BoardPos, GameMove, BoardState };
use game::game_rules::GameRules;

pub enum PlayerColor {
    Black,
    White
}

pub enum GameResult {
    Win(PlayerColor),
    Tie,
}

pub struct BoardGame<T: GameRules> {
    player_turn: PlayerColor,
    score: u32,
    rules: GameRules,
    boardstate: BoardState<T>,
    // move_history: Vec<Move> a history of moves?
}

impl <T> BoardGame<T> {
    fn WithRules<T: GameRules>(rules: T) -> Self {
        BoardGame{ rules: rules }
    }

    fn whose_turn(&self) -> PlayerColor {
        self.player_turn
    }

    fn reset(&self) {}

    fn legal_moves(&self, player: PlayerColor) -> Vec<BoardPos> {
        Vec::new()
    }

    fn boardstate(&self) -> &BoardState<T> {
        &self.boardstate
    }

    fn apply_move(&self, pos: BoardPos, piece: Piece) {

    }

    fn winner(&self) -> GameResult {
        GameResult::Win(PlayerColor::Black)
    }
}


// design time
// we have some abstract concepts:
// - a Game
//      - two agents (players)
//      - a winner
//      - the current player's turn
//      - the state of the board
// - a Board<T>
//      - holds a fixed-sized array of Pieces<T>
// - Piece
//      - Piece<T> is a holder, where T may be an enum like 'ReversiPiece', black or white
// - a GameAgent
// and we have:
// - a concrete GameRunner, that knows how to run any game
//      - put simply, agents (players) take turns until the game is over.
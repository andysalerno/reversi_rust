use game::board::{ Piece, GameMove, BoardState };

pub enum WinResult {
    White,
    Black,
    Tie,
}

pub enum PlayerColor {
    White,
    Black
}

pub trait GameAgent<T> 
where T: Piece<T> {
    fn pick_move<'a>(&self, board: &BoardState<T>, choices: &'a [GameMove<T>]) -> &'a GameMove<T>;
    fn color(&self) -> PlayerColor;
}

pub trait BoardGame<T> {
    fn winner(&self) -> Option<WinResult>;
    fn take_turn(&self, agent: GameAgent<T>);
    fn whose_turn(&self) -> PlayerColor;
    fn get_boardstate(&self) -> &BoardState<T>;
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
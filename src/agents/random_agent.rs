use game::boardgame::GameAgent;
use game::board::Piece;

pub struct RandomAgent;

impl <T: Piece<T>> GameAgent<T> for RandomAgent {

}